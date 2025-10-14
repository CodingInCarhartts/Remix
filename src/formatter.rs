use crate::config::OutputConfig;
use crate::packer::PackedRepository;
use crate::utils::{format_size, open_file};
use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::security::SecurityCheckStatus;

pub fn output_result(repo: &PackedRepository, config: &OutputConfig) -> Result<()> {
    let output_path = &config.path;
    let format = &config.format;

    info!("Generating output in {} format to {}", format, output_path);

    let content = match format.as_str() {
        "md" | "markdown" => format_markdown(repo),
        "json" => format_json(repo)?,
        "txt" | "text" => format_text(repo),
        _ => {
            warn!("Unknown format '{}', defaulting to markdown", format);
            format_markdown(repo)
        }
    };

    // Write the output to a file
    let mut file = fs::File::create(output_path)
        .context(format!("Failed to create output file: {}", output_path))?;

    file.write_all(content.as_bytes())
        .context(format!("Failed to write to output file: {}", output_path))?;

    info!("Output written to {}", output_path);

    // Open the file if requested
    if config.open_file {
        info!("Opening output file");
        open_file(output_path).context(format!("Failed to open output file: {}", output_path))?;
    }

    Ok(())
}

fn format_markdown(repo: &PackedRepository) -> String {
    let mut output = String::new();

    // Add user instruction if provided
    if let Some(instruction) = &repo.instruction {
        output.push_str("# User Instruction\n\n");
        output.push_str(instruction);
        output.push_str("\n\n");
    }

    // Add repository summary
    output.push_str("# Repository Summary\n\n");
    output.push_str(&format!("- **Files:** {}\n", repo.summary.file_count));
    output.push_str(&format!(
        "- **Directories:** {}\n",
        repo.summary.directory_count
    ));
    output.push_str(&format!(
        "- **Total Size:** {}\n",
        format_size(repo.summary.total_size)
    ));
    output.push_str(&format!(
        "- **Binary Files:** {}\n",
        repo.summary.binary_file_count
    ));

    if !repo.summary.extensions.is_empty() {
        output.push_str(&format!(
            "- **Extensions:** {}\n",
            repo.summary.extensions.join(", ")
        ));
    }

    // Add security check results if available
    match &repo.security_check_status {
        SecurityCheckStatus::Disabled => {
            output.push_str("## Security Check\n\n");
            output.push_str("🔒 **Security check was disabled**\n");
        }
        SecurityCheckStatus::CompletedNoFindings => {
            output.push_str("## Security Check\n\n");
            output.push_str("✅ **Security check completed - no suspicious files found**\n");
        }
        SecurityCheckStatus::CompletedWithFindings => {
            output.push_str("\n## Security Check Results\n\n");
            output.push_str(&format!(
                "⚠️ **{} suspicious file(s) detected that may contain sensitive information:**\n\n",
                repo.suspicious_files.as_ref().map_or(0, |v| v.len())
            ));

            for (i, file) in repo.suspicious_files.as_ref().unwrap_or(&vec![]).iter().enumerate() {
                output.push_str(&format!("{}. `{}`\n", i + 1, file));
            }

            output
                .push_str("\n> **Note:** Please review these files before sharing this output.\n");
        }
        SecurityCheckStatus::Failed(error) => {
            output.push_str("## Security Check\n\n");
            output.push_str(&format!("❌ **Security check failed**: {}\n", error));
        }
    }

    // Add binary files list if available
    if let Some(binary_files) = &repo.binary_files {
        if !binary_files.is_empty() {
            output.push_str("\n## Binary Files\n\n");
            output.push_str(
                "The following binary files were detected but not included in the content:\n\n",
            );

            for (i, file) in binary_files.iter().enumerate() {
                output.push_str(&format!("{}. `{}`\n", i + 1, file));
            }
        }
    }

    output.push_str("\n# Files\n\n");

    // Group files by directory
    let mut files_by_dir: std::collections::BTreeMap<String, Vec<&crate::packer::FileContent>> =
        std::collections::BTreeMap::new();

    for file in &repo.files {
        let path = Path::new(&file.relative_path);
        let parent = path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        files_by_dir.entry(parent).or_default().push(file);
    }

    // Output files by directory
    for (dir, files) in &files_by_dir {
        if dir != "." {
            output.push_str(&format!("## {}/\n\n", dir));
        } else {
            output.push_str("## Root Directory\n\n");
        }

        for file in files {
            let filename = Path::new(&file.relative_path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| file.relative_path.clone());

            output.push_str(&format!("### {}\n\n", filename));

            // Add file metadata
            output.push_str(&format!("- **Path:** {}\n", file.relative_path));
            output.push_str(&format!("- **Size:** {}\n", format_size(file.size)));

            if !file.extension.is_empty() {
                output.push_str(&format!("- **Type:** {}\n", file.extension));
            }

            output.push_str("\n```");

            // Add language hint for syntax highlighting
            if !file.extension.is_empty() {
                output.push_str(&file.extension);
            }

            output.push('\n');
            output.push_str(&file.content);
            output.push('\n');
            output.push_str("```\n\n");
        }
    }

    output
}

fn format_json(repo: &PackedRepository) -> Result<String> {
    serde_json::to_string_pretty(repo).context("Failed to serialize repository to JSON")
}

fn format_text(repo: &PackedRepository) -> String {
    let mut output = String::new();

    // Add user instruction if provided
    if let Some(instruction) = &repo.instruction {
        output.push_str("USER INSTRUCTION:\n\n");
        output.push_str(instruction);
        output.push_str("\n\n");
    }

    // Add repository summary
    output.push_str("REPOSITORY SUMMARY:\n\n");
    output.push_str(&format!("Files: {}\n", repo.summary.file_count));
    output.push_str(&format!("Directories: {}\n", repo.summary.directory_count));
    output.push_str(&format!(
        "Total Size: {}\n",
        format_size(repo.summary.total_size)
    ));
    output.push_str(&format!(
        "Binary Files: {}\n",
        repo.summary.binary_file_count
    ));

    if !repo.summary.extensions.is_empty() {
        output.push_str(&format!(
            "Extensions: {}\n",
            repo.summary.extensions.join(", ")
        ));
    }

    // Add security check results if available
    match &repo.security_check_status {
        SecurityCheckStatus::Disabled => {
            output.push_str("SECURITY CHECK:\n\n");
            output.push_str("Security check was disabled.\n\n");
        }
        SecurityCheckStatus::CompletedNoFindings => {
            output.push_str("SECURITY CHECK:\n\n");
            output.push_str("Security check completed - no suspicious files found.\n\n");
        }
        SecurityCheckStatus::CompletedWithFindings => {
            output.push_str("SECURITY CHECK:\n\n");
            output.push_str("WARNING: ");
            output.push_str(&format!("{} suspicious file(s) detected that may contain sensitive information:\n\n", repo.suspicious_files.as_ref().map_or(0, |v| v.len())));

            for (i, file) in repo.suspicious_files.as_ref().unwrap_or(&vec![]).iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, file));
            }

            output.push_str("\nPlease review these files before sharing this output.\n");
        }
        SecurityCheckStatus::Failed(error) => {
            output.push_str("SECURITY CHECK:\n\n");
            output.push_str(&format!("Security check failed: {}\n", error));
        }
    }

    // Add binary files list if available
    if let Some(binary_files) = &repo.binary_files {
        if !binary_files.is_empty() {
            output.push_str("\nBINARY FILES:\n\n");
            output.push_str(
                "The following binary files were detected but not included in the content:\n\n",
            );

            for (i, file) in binary_files.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, file));
            }
        }
    }

    output.push_str("\nFILES:\n\n");

    // Output each file
    for file in &repo.files {
        output.push_str(&format!("FILE: {}\n", file.relative_path));
        output.push_str(&format!("SIZE: {}\n", format_size(file.size)));

        if !file.extension.is_empty() {
            output.push_str(&format!("TYPE: {}\n", file.extension));
        }

        output.push_str("\nCONTENT:\n");
        output.push_str(&file.content);
        output.push_str("\n\n");
        output.push_str("--------------------------------\n\n");
    }

    output
}

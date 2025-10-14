use crate::comments;
use crate::config::Config;
use crate::scanner::{scan_repository, FileInfo};
use crate::security;
use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, info, warn};
use rayon::prelude::*;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Clone)]
pub struct FileContent {
    pub relative_path: String,
    pub extension: String,
    pub content: String,
    pub size: u64,
    pub is_binary: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct PackedRepository {
    pub files: Vec<FileContent>,
    pub summary: RepositorySummary,
    pub instruction: Option<String>,
    pub suspicious_files: Option<Vec<String>>,
    pub security_check_status: security::SecurityCheckStatus,  // NEW: Track security check status
    pub binary_files: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RepositorySummary {
    pub file_count: usize,
    pub total_size: u64,
    pub directory_count: usize,
    pub extensions: Vec<String>,
    pub binary_file_count: usize,
}

pub async fn pack_repository(path: &Path, config: &Config) -> Result<PackedRepository> {
    info!("Packing repository at {}", path.display());

    // Create a multi-progress bar for tracking multiple processes
    let multi_progress = MultiProgress::new();

    // Scan the repository to find all files
    let scan_progress = multi_progress.add(ProgressBar::new_spinner());
    scan_progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {prefix:.bold.dim} {msg}")
            .unwrap(),
    );
    scan_progress.set_prefix("[Scan]");
    scan_progress.set_message("Scanning repository...");

    let files = scan_repository(path, config)?;
    scan_progress.finish_with_message(format!("Found {} files", files.len()));

    debug!("Found {} files to process", files.len());

    // Track binary files separately
    let binary_files: Vec<String> = files
        .iter()
        .filter(|file| file.is_binary)
        .map(|file| file.relative_path.to_string_lossy().to_string())
        .collect();

    // Create a progress bar for processing files
    let process_progress = multi_progress.add(ProgressBar::new(files.len() as u64));
    process_progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} {prefix:.bold.dim} [{bar:40.cyan/blue}] {pos}/{len} files {msg}",
            )
            .unwrap()
            .progress_chars("=> "),
    );
    process_progress.set_prefix("[Process]");
    process_progress.set_message("Processing files...");

    // We need to wrap the progress bar in an Arc<Mutex<>> for thread-safe access
    let progress = Arc::new(Mutex::new(process_progress));

    // Process files in parallel
    let file_contents: Vec<FileContent> = files
        .par_iter()
        .filter_map(|file| {
            let result = match read_file_content(file, config) {
                Ok(Some(content)) => Some(content),
                Ok(none) => none,
                Err(e) => {
                    warn!("Error reading file {}: {}", file.path.display(), e);
                    None
                }
            };

            // Update the progress bar
            if let Ok(pb) = progress.lock() {
                pb.inc(1);
                if let Some(content) = &result {
                    pb.set_message(format!("Processed {}", content.relative_path));
                }
            }

            result
        })
        .collect();

    // Finish the progress bar
    if let Ok(pb) = progress.lock() {
        pb.finish_with_message(format!("Processed {} files", file_contents.len()));
    }

    info!("Processed {} files", file_contents.len());

    // Perform security check if enabled
    let security_progress = multi_progress.add(ProgressBar::new_spinner());
    security_progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.yellow} {prefix:.bold.dim} {msg}")
            .unwrap(),
    );
    security_progress.set_prefix("[Security]");

    let (suspicious_files, security_status) = if !config.security.enable_security_check {
        security_progress.finish_with_message("Security check disabled");
        (None, security::SecurityCheckStatus::Disabled)
    } else {
        security_progress.set_message("Performing security check...");
        match security::perform_security_check(path) {
            Ok(files) => {
                if !files.is_empty() {
                    security_progress
                        .finish_with_message(format!("Found {} suspicious files", files.len()));
                    info!(
                        "Found {} suspicious files that may contain sensitive information",
                        files.len()
                    );
                    (Some(files), security::SecurityCheckStatus::CompletedWithFindings)
                } else {
                    security_progress.finish_with_message("No suspicious files found");
                    (None, security::SecurityCheckStatus::CompletedNoFindings)
                }
            }
            Err(e) => {
                security_progress.finish_with_message(format!("Security check failed: {}", e));
                warn!("Security check failed: {}", e);
                (None, security::SecurityCheckStatus::Failed(e.to_string()))
            }
        }
    };

    // Generate a summary of the repository
    let summary = generate_summary(&file_contents, binary_files.len());

    // Read custom instruction file if provided
    let instruction = match &config.output.instruction_file_path {
        Some(instruction_file) => {
            let path = Path::new(instruction_file);
            if path.exists() {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        debug!("Read instruction file: {}", instruction_file);
                        Some(content)
                    }
                    Err(e) => {
                        warn!(
                            "Failed to read instruction file {}: {}",
                            instruction_file, e
                        );
                        config.instruction.clone()
                    }
                }
            } else {
                warn!("Instruction file not found: {}", instruction_file);
                config.instruction.clone()
            }
        }
        _none => config.instruction.clone(),
    };

    Ok(PackedRepository {
        files: file_contents,
        summary,
        instruction,
        suspicious_files,  // Now properly tracks security check results
        security_check_status: security_status,  // Fix: use correct variable name
        binary_files: Some(binary_files),
    })
}

fn read_file_content(file: &FileInfo, config: &Config) -> Result<Option<FileContent>> {
    // Don't try to read binary files unless they were explicitly included
    if file.is_binary {
        debug!("Skipping binary file: {}", file.path.display());
        return Ok(None);
    }

    // Read the file content
    let content = fs::read_to_string(&file.path)
        .context(format!("Failed to read file: {}", file.path.display()))?;

    // Check for sensitive content if security check is enabled
    if config.security.enable_security_check && security::check_sensitive_content(&content) {
        warn!(
            "Skipping file with sensitive content: {}",
            file.path.display()
        );
        return Ok(None);
    }

    // Get the file extension
    let extension = file
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Process the content (compression or comment removal)
    let processed_content = if config.compress {
        compress_content(&content, &extension)
    } else if config.output.remove_comments && comments::is_comment_removal_supported(&extension) {
        comments::remove_comments(&content, &extension)
    } else {
        content
    };

    Ok(Some(FileContent {
        relative_path: file.relative_path.to_string_lossy().to_string(),
        extension,
        content: processed_content,
        size: file.size,
        is_binary: file.is_binary,
    }))
}

fn compress_content(content: &str, _extension: &str) -> String {
    // This is a basic implementation for code compression
    // In a full implementation, you would use a proper parser for each language
    // and extract only the structural elements like function signatures, class declarations, etc.

    // For now, let's just do some basic line-based filtering
    let lines: Vec<&str> = content.lines().collect();

    // Skip compression for small files
    if lines.len() < 10 {
        return content.to_string();
    }

    // Basic compression by taking first line of blocks and removing empty lines
    let mut compressed = Vec::new();
    let mut in_comment_block = false;
    let mut consecutive_empty_lines = 0;

    for line in lines {
        let trimmed = line.trim();

        // Handle comment blocks
        if trimmed.starts_with("/*") || trimmed.starts_with("/**") {
            in_comment_block = true;
            compressed.push(line);
            continue;
        }

        if in_comment_block {
            if trimmed.ends_with("*/") {
                in_comment_block = false;
                compressed.push(line);
            }
            continue;
        }

        // Skip empty lines after the first one
        if trimmed.is_empty() {
            consecutive_empty_lines += 1;
            if consecutive_empty_lines <= 1 {
                compressed.push(line);
            }
            continue;
        }
        consecutive_empty_lines = 0;

        // Skip comment lines
        if trimmed.starts_with("//") {
            continue;
        }

        // Always include certain structural elements
        if trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("class ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("pub struct ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("export ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("use ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("let ")
            || trimmed.starts_with("var ")
            || trimmed.starts_with("function ")
        {
            compressed.push(line);
            continue;
        }

        // Include opening and closing braces
        if trimmed == "{" || trimmed == "}" {
            compressed.push(line);
            continue;
        }

        // For implementation blocks, only include the first line
        if trimmed.contains("impl") || trimmed.contains(" for ") {
            compressed.push(line);
            continue;
        }

        // Skip most implementation details
        if !trimmed.contains("(") && !trimmed.contains(")") {
            continue;
        }

        compressed.push(line);
    }

    compressed.join("\n")
}

fn generate_summary(files: &[FileContent], binary_file_count: usize) -> RepositorySummary {
    let file_count = files.len();
    let total_size: u64 = files.iter().map(|f| f.size).sum();

    // Count directories (unique parent paths)
    let mut directories = std::collections::HashSet::new();

    for file in files {
        let path = PathBuf::from(&file.relative_path);
        if let Some(parent) = path.parent() {
            directories.insert(parent.to_path_buf());
        }
    }

    let directory_count = directories.len();

    // Count extensions
    let mut extension_counts = std::collections::HashMap::new();

    for file in files {
        if !file.extension.is_empty() {
            *extension_counts.entry(file.extension.clone()).or_insert(0) += 1;
        }
    }

    let mut extensions: Vec<String> = extension_counts.keys().cloned().collect();
    extensions.sort();

    RepositorySummary {
        file_count,
        total_size,
        directory_count,
        extensions,
        binary_file_count,
    }
}

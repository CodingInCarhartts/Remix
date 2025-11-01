use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "remix",
    about = "Pack your repository into a single file for AI tools",
    version,
    author
)]
pub struct Cli {
    /// Path to the directory to process
    #[arg(index = 1)]
    pub path: Option<String>,

    /// Path to a configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Initialize a new configuration file
    #[arg(long)]
    pub init: bool,

    /// Include files matching the glob pattern (comma-separated)
    #[arg(long)]
    pub include: Option<String>,

    /// Exclude files matching the glob pattern (comma-separated)
    #[arg(long)]
    pub ignore: Option<String>,

    /// Maximum file size in bytes to include
    #[arg(long)]
    pub max_file_size: Option<u64>,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format (md, json, txt, toon)
    #[arg(long, value_parser = ["md", "markdown", "json", "txt", "text", "toon"])]
    pub format: Option<String>,

    /// Compress the code output
    #[arg(long)]
    pub compress: bool,

    /// Skip checking for sensitive information
    #[arg(long)]
    pub skip_sensitive_check: bool,

    /// Remote repository URL
    #[arg(long)]
    pub remote: Option<String>,

    /// Branch name, tag, or commit hash for remote repository
    #[arg(long)]
    pub remote_branch: Option<String>,

    /// Open output file after generation
    #[arg(long)]
    pub open: bool,

    /// Add a user-specified instruction at the top of the output
    #[arg(long)]
    pub instruction: Option<String>,

    /// Path to a file containing detailed instructions or context
    #[arg(long)]
    pub instruction_file: Option<PathBuf>,

    /// Remove comments from supported file types
    #[arg(long)]
    pub remove_comments: bool,

    /// Don't use patterns from .gitignore files
    #[arg(long)]
    pub no_gitignore: bool,

    /// Don't use default ignore patterns
    #[arg(long)]
    pub no_default_patterns: bool,
}

impl Cli {
    /// Converts a comma-separated string to a vector of strings
    pub fn parse_comma_separated(&self, input: &Option<String>) -> Option<Vec<String>> {
        input.as_ref().map(|s| {
            s.split(',')
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect()
        })
    }

    /// Get include patterns as a vector
    pub fn include_patterns(&self) -> Option<Vec<String>> {
        self.parse_comma_separated(&self.include)
    }

    /// Get ignore patterns as a vector
    pub fn ignore_patterns(&self) -> Option<Vec<String>> {
        self.parse_comma_separated(&self.ignore)
    }
}

# User Instruction

Please analyze this Rust codebase for performance optimizations and security vulnerabilities. Focus on the core logic in src/ and ignore test files unless they demonstrate important patterns.

# Repository Summary

- **Files:** 17
- **Directories:** 3
- **Total Size:** 87.76 KB
- **Binary Files:** 0
- **Extensions:** md, rs, toml

## Security Check Results

⚠️ **6 suspicious file(s) detected that may contain sensitive information:**

1. `tests/security_tests.rs`
2. `src/config.rs`
3. `tests/common/mod.rs`
4. `src/security.rs`
5. `tests/config_tests.rs`
6. `remix.config.json`

> **Note:** Please review these files before sharing this output.

# Files

## /

### Cargo.toml

- **Path:** Cargo.toml
- **Size:** 966 bytes
- **Type:** toml

```toml
[package]
name = "remix"
version = "0.1.0"
edition = "2021"
authors = ["Trenton Sousa tdsousa1993@gmail.com"]
description = "A Rust implementation of repomix - pack repositories for AI tools"
license = "MIT"
readme = "README.md"
repository = "https://github.com/CodingInCarhartts/remix"
keywords = ["ai", "repository", "packing", "llm"]

[dependencies]
clap = { version = "4.4", features = ["derive"] }
glob = "0.3"
walkdir = "2.4"
ignore = "0.4"
regex = "1.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.36", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1.0"
thiserror = "1.0"
log = "0.4"
env_logger = "0.10"
indicatif = "0.17"
console = "0.15"
tempfile = "3.9"
git2 = "0.18"
crossbeam = "0.8"
rayon = "1.8"
tree_magic_mini = "3.0"
glob-match = "0.2"
rtoon = "0.1.3"

[dev-dependencies]
tempfile = "3.9"
test-case = "3.3"
mockall = "0.12"
assert_fs = "1.1"
predicates = "3.0"

```

### README.md

- **Path:** README.md
- **Size:** 10.35 KB
- **Type:** md

```md
<div align="center">

# 🚀 Remix

[![Crates.io](https://img.shields.io/crates/v/remix.svg)](https://crates.io/crates/remix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)

**A blazing-fast Rust implementation of [repomix](https://github.com/yamadashy/repomix)**

*Pack your entire repository into a single, AI-friendly file*

[Installation](#-installation) •
[Usage](#-usage) •
[Configuration](#-configuration) •
[Features](#-features) •
[Examples](#-examples) •
[License](#-license)

</div>

---

## 📖 Overview

Remix prepares your codebase for AI analysis by packing it into a single file. Perfect for feeding your code to Large Language Models (LLMs) like Claude, ChatGPT, DeepSeek, Perplexity, Gemini, and more.


 ```mermaid
flowchart TD
    A[CLI Execution] --> B[Parse Command Line Arguments]
    B --> C[Load Configuration]
    C --> D{Remote Repository?}
    
    D -->|Yes| E[Parse Remote URL]
    E --> F[Clone Repository to Temp Directory]
    F --> G[Checkout Target Branch/Commit]
    
    D -->|No| H[Use Local Directory Path]
    
    G --> I[Scan Repository]
    H --> I
    
    I --> J[Apply Include/Ignore Filters]
    J --> K[Multi-layer Filtering]
    K --> L[File Size Check]
    L --> M[Binary File Detection]
    
    M --> N[Parallel File Processing]
    N --> O[Read File Content]
    O --> P{Security Check Enabled?}
    P -->|Yes| Q[Check for Sensitive Content]
    P -->|No| R[Skip Security Check]
    
    Q --> S[Filter Sensitive Files]
    R --> T[Process File Content]
    S --> T
    
    T --> U{Compression Mode?}
    U -->|Yes| V[Compress Content]
    U -->|No| W{Comment Removal?}
    
    V --> X[Format Output]
    W -->|Yes| Y[Remove Comments]
    W -->|No| X
    
    Y --> X
    
    X --> Z[Generate Repository Summary]
    Z --> AA[Format Output File]
    AA --> BB{Markdown Format}
    AA --> CC{JSON Format}
    AA --> DD{Text Format}
    
    BB --> EE[Write Markdown File]
    CC --> FF[Write JSON File]
    DD --> GG[Write Text File]
    
    EE --> HH{Open File?}
    FF --> HH
    GG --> HH
    
    HH -->|Yes| II[Open Output File]
    HH -->|No| JJ[Workflow Complete]
    II --> JJ
```
<div align="center">
  <p><i>A visualization of how Remix processes your repository</i></p>
</div>

## ✨ Features

| Feature | Description |
|---------|-------------|
| ⚡ **High Performance** | Built in Rust for maximum speed and efficiency |
| 📁 **Repository Packing** | Combine your entire codebase into a single file |
| 🌐 **Remote Repository Support** | Process GitHub repositories directly with branch/tag/commit support |
| 🎯 **Intelligent Filtering** | Include/exclude files using glob patterns |
| 🛡️ **Multi-layered Ignore System** | Uses `.gitignore`, `.mixignore`, and custom ignore patterns |
| 🔒 **Security Checks** | Automatically detect and warn about sensitive information |
| 📝 **Multiple Output Formats** | Markdown, JSON, plain text, and TOON support |
| 🧹 **Comment Removal** | Optionally strip comments from source code to reduce token count |
| ⚙️ **Flexible Configuration** | JSON-based config files with CLI overrides |
| 🎨 **AI Tool Optimized** | Formatted output designed for LLM consumption |

## 📦 Installation

**From Crates.io:**
```bash
cargo install remix
```

**From Source:**
```bash
git clone https://github.com/CodingInCarhartts/remix.git
cd remix
cargo build --release
# Binary will be available at target/release/remix
```

**As a Cargo Subcommand:**
```bash
cargo install --features cargo-subcommand remix
# Then use as: cargo mix [options]
```

## 🔧 Usage

### Basic Commands

Pack your entire repository:
```bash
remix
```

Pack a specific directory:
```bash
remix /path/to/your/project
```

Pack a remote repository:
```bash
remix --remote https://github.com/username/repo
```

### Advanced Options

<details>
<summary>📂 <b>File Selection</b></summary>

```bash
# Include specific files or directories using glob patterns
remix --include "*.rs,*.toml"
remix --include "src/**/*.rs,tests/**/*.rs"

# Exclude specific files or directories
remix --ignore "*.log,*.tmp"
remix --ignore "node_modules/**,target/**"

# Set maximum file size (in bytes)
remix --max-file-size 50000
```
</details>

<details>
<summary>🌐 <b>Remote Repositories</b></summary>

```bash
# Pack a remote repository (full URL)
remix --remote https://github.com/microsoft/vscode

# Pack a specific branch
remix --remote https://github.com/user/repo --remote-branch develop

# Pack a specific tag
remix --remote https://github.com/user/repo --remote-branch v1.2.3

# Using GitLab
remix --remote https://gitlab.com/example/project
```
</details>

<details>
<summary>⚙️ <b>Processing Options</b></summary>

```bash
# Remove comments from code
remix --remove-comments

# Skip security checks (use with caution)
remix --skip-sensitive-check

# Compress the output
remix --compress

# Add custom instructions for AI
remix --instruction "Please analyze this Rust codebase"

# Use instruction file
remix --instruction-file ./context.txt

# Initialize a new configuration file
remix --init
```
</details>

<details>
<summary>📄 <b>Output Options</b></summary>

```bash
# Specify output path
remix --output ./my-repo.md

# Change output format (md, json, txt, toon)
remix --format toon

# Open output file after generation
remix --open
```
</details>

## 📝 Configuration

Create a `remix.config.json` file in your project root for custom configurations:

```json
{
  "include": [],
  "ignore": {
    "use_gitignore": true,
    "use_default_patterns": true,
    "use_mixignore": true,
    "custom_patterns": []
  },
  "max_file_size": 100000,
  "compress": false,
  "security": {
    "enable_security_check": true
  },
  "output": {
    "format": "md",
    "open_file": false,
    "path": "./remix-output.md",
    "instruction_file_path": null,
    "remove_comments": false
  },
  "instruction": null
}
```

### Configuration Examples

<details>
<summary>Basic configuration with custom includes</summary>

```json
{
  "include": ["src/**/*.rs", "Cargo.toml", "README.md"],
  "output": {
    "path": "./my-project.md"
  }
}
```
</details>

<details>
<summary>Configuration for a Node.js project</summary>

```json
{
  "include": ["src/**/*.js", "package.json", "README.md"],
  "ignore": {
    "custom_patterns": ["node_modules/**", "*.log"]
  },
  "output": {
    "format": "md",
    "remove_comments": true
  }
}
```
</details>

<details>
<summary>Configuration for AI analysis</summary>

```json
{
  "include": ["src/**/*.py", "requirements.txt", "docs/**"],
  "max_file_size": 50000,
  "output": {
    "format": "md",
    "instruction_file_path": "./ai-instructions.txt"
  },
  "instruction": "Please analyze this Python codebase for security vulnerabilities"
}
```
</details>

## 🚫 Ignore Files

Remix uses a multi-layered ignore system:

1. **`.gitignore`** - Standard Git ignore patterns are respected
2. **`.mixignore`** - Project-specific ignore patterns for Remix
3. **Command line `--ignore` patterns** or configuration file settings
4. **Default patterns** - Common files and directories (node_modules, target, etc.)

You can disable any of these layers using command-line options:
- `--no-gitignore` - Don't use .gitignore patterns
- `--no-default-patterns` - Don't use default ignore patterns

## 📊 Output Formats

### Markdown (Default)

Generates a well-formatted markdown file with:
- File tree structure
- Syntax-highlighted code blocks
- File metadata (size, path)
- Instructions and context at the top

### JSON

Structured JSON output containing:
- Repository metadata
- File contents as base64-encoded strings
- File information (path, size, type)
- Configuration used

### TOON

Token-efficient output using the TOON format:
- Compact representation optimized for LLM prompts
- 30-60% fewer tokens than JSON for structured data
- Human-readable with indentation-based structure
- Ideal for AI analysis workflows

### Text

Plain text output with:
- Simple file headers
- Raw code content
- Minimal formatting

### Basic Usage Examples

**Pack the current directory:**
```bash
remix
```

**Pack a specific project:**
```bash
remix ~/projects/my-app
```

**Include only Rust files:**
```bash
remix --include "*.rs"
```

### AI Tool Preparation

**Add instructions for AI analysis:**
```bash
remix --instruction "Please review this codebase for security issues" --output ./security-review.md
```

**Use instruction file:**
```bash
echo "Analyze the following Rust code for performance optimizations:" > instructions.txt
remix --instruction-file ./instructions.txt --include "*.rs"
```

### Advanced Filtering

**Process only small files:**
```bash
remix --max-file-size 10000
```

**Exclude build artifacts:**
```bash
remix --ignore "target/**,*.log"
```

## 🔧 Troubleshooting

### Common Issues

**"No files found" error:**
- Check your include patterns
- Verify the path exists and contains files
- Try `--no-default-patterns` if files are being excluded

**Large output files:**
- Use `--max-file-size` to limit file sizes
- Add more ignore patterns with `--ignore`
- Use `--compress` to reduce output size

**Permission denied:**
- Ensure you have read access to the target directory
- For remote repos, ensure the repository is public or you have access

**Configuration not loading:**
- Verify the JSON syntax is valid
- Check the config file path
- Use `--init` to create a new config file

### Performance Tips

- Use specific include patterns instead of processing entire directories
- Exclude large directories like `node_modules`, `target`, etc.
- Use `--compress` for smaller output files
- For large repositories, consider using `--max-file-size`

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

[MIT License](https://opensource.org/licenses/MIT) - See the LICENSE file for details.

---

<div align="center">
  <p>Built with ❤️ by CodingInCarhartts of the Rust community</p>
  <p>
    <a href="https://github.com/CodingInCarhartts/remix/issues">Report Bug</a> •
    <a href="https://github.com/CodingInCarhartts/remix/issues">Request Feature</a>
  </p>
</div>

```

## src/

### cli.rs

- **Path:** src/cli.rs
- **Size:** 2.67 KB
- **Type:** rs

```rs
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
    
    #[arg(index = 1)]
    pub path: Option<String>,

    
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    
    #[arg(long)]
    pub init: bool,

    
    #[arg(long)]
    pub include: Option<String>,

    
    #[arg(long)]
    pub ignore: Option<String>,

    
    #[arg(long)]
    pub max_file_size: Option<u64>,

    
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    
    #[arg(long, value_parser = ["md", "markdown", "json", "txt", "text", "toon"])]
    pub format: Option<String>,

    
    #[arg(long)]
    pub compress: bool,

    
    #[arg(long)]
    pub skip_sensitive_check: bool,

    
    #[arg(long)]
    pub remote: Option<String>,

    
    #[arg(long)]
    pub remote_branch: Option<String>,

    
    #[arg(long)]
    pub open: bool,

    
    #[arg(long)]
    pub instruction: Option<String>,

    
    #[arg(long)]
    pub instruction_file: Option<PathBuf>,

    
    #[arg(long)]
    pub remove_comments: bool,

    
    #[arg(long)]
    pub no_gitignore: bool,

    
    #[arg(long)]
    pub no_default_patterns: bool,
}

impl Cli {
    
    pub fn parse_comma_separated(&self, input: &Option<String>) -> Option<Vec<String>> {
        input.as_ref().map(|s| {
            s.split(',')
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect()
        })
    }

    
    pub fn include_patterns(&self) -> Option<Vec<String>> {
        self.parse_comma_separated(&self.include)
    }

    
    pub fn ignore_patterns(&self) -> Option<Vec<String>> {
        self.parse_comma_separated(&self.ignore)
    }
}

```

### comments.rs

- **Path:** src/comments.rs
- **Size:** 12.19 KB
- **Type:** rs

```rs

pub fn remove_comments(content: &str, file_extension: &str) -> String {
    match file_extension {
        "rs" => remove_rust_comments(content),
        "js" | "ts" | "jsx" | "tsx" => remove_js_comments(content),
        "py" => remove_python_comments(content),
        "c" | "cpp" | "h" | "hpp" | "cs" | "java" | "go" | "swift" | "kt" => {
            remove_c_style_comments(content)
        }
        "rb" => remove_ruby_comments(content),
        "php" => remove_php_comments(content),
        "html" | "xml" | "svg" => remove_html_comments(content),
        "css" | "scss" | "sass" | "less" => remove_css_comments(content),
        "sh" | "bash" => remove_shell_comments(content),
        "yaml" | "yml" => remove_yaml_comments(content),
        _ => content.to_string(),
    }
}


pub fn is_comment_removal_supported(extension: &str) -> bool {
    matches!(
        extension,
        "rs" | "js"
            | "ts"
            | "jsx"
            | "tsx"
            | "py"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "cs"
            | "java"
            | "go"
            | "swift"
            | "kt"
            | "rb"
            | "php"
            | "html"
            | "xml"
            | "svg"
            | "css"
            | "scss"
            | "sass"
            | "less"
            | "sh"
            | "bash"
            | "yaml"
            | "yml"
    )
}


fn remove_rust_comments(content: &str) -> String {
    remove_c_style_comments(content) 
}


fn remove_js_comments(content: &str) -> String {
    remove_c_style_comments(content) 
}


fn remove_python_comments(content: &str) -> String {
    let mut result = String::new();
    let mut in_multiline_string = false;
    let mut multiline_quotes = "";

    let lines = content.lines();
    for line in lines {
        let line_trim = line.trim();

        
        if in_multiline_string {
            result.push_str(line);
            result.push('\n');

            if line.contains(multiline_quotes) {
                
                let parts: Vec<&str> = line.rsplitn(2, multiline_quotes).collect();
                if parts.len() > 1 && !parts[0].ends_with('\\') {
                    in_multiline_string = false;
                }
            }
            continue;
        }

        
        if (line_trim.contains("'''") || line_trim.contains("\"\"\""))
            && !(line_trim.starts_with("#"))
        {
            if line_trim.contains("'''") {
                multiline_quotes = "'''";
            } else {
                multiline_quotes = "\"\"\"";
            }

            
            let count = line_trim.matches(multiline_quotes).count();
            if count % 2 == 1 {
                in_multiline_string = true;
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            
            let in_string = preceding.chars().filter(|&c| c == '"' || c == '\'').count() % 2 == 1;

            if in_string {
                result.push_str(line);
            } else {
                result.push_str(preceding);
            }
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}


fn remove_c_style_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut in_char = false;
    let mut in_single_line_comment = false;
    let mut in_multi_line_comment = false;
    let mut string_quote = '"';

    while let Some(c) = chars.next() {
        match c {
            
            '"' | '\'' => {
                if !in_single_line_comment && !in_multi_line_comment {
                    if c == '"' && !in_char {
                        if !in_string {
                            string_quote = '"';
                            in_string = true;
                        } else if string_quote == '"' {
                            in_string = false;
                        }
                        result.push(c);
                    } else if c == '\'' && !in_string {
                        in_char = !in_char;
                        result.push(c);
                    } else {
                        result.push(c);
                    }
                }
            }
            
            '\\' => {
                if !in_single_line_comment && !in_multi_line_comment && (in_string || in_char) {
                    result.push(c);
                    if let Some(next) = chars.next() {
                        result.push(next);
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            
            '/' => {
                if !in_string && !in_char && !in_single_line_comment && !in_multi_line_comment {
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            in_single_line_comment = true;
                            chars.next(); 
                        } else if next == '*' {
                            in_multi_line_comment = true;
                            chars.next(); 
                        } else {
                            result.push(c);
                        }
                    } else {
                        result.push(c);
                    }
                } else if in_multi_line_comment {
                    if let Some(prev) = result.chars().last() {
                        if prev == '*' {
                            result.pop(); 
                            in_multi_line_comment = false;
                        }
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            
            '\n' => {
                if in_single_line_comment {
                    in_single_line_comment = false;
                }
                result.push(c);
            }
            
            '*' => {
                if !in_string && !in_char && !in_single_line_comment && in_multi_line_comment {
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            in_multi_line_comment = false;
                            chars.next(); 
                        }
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            
            _ => {
                if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
        }
    }

    result
}


fn remove_ruby_comments(content: &str) -> String {
    
    remove_python_comments(content)
}


fn remove_php_comments(content: &str) -> String {
    
    let content = remove_c_style_comments(content);

    
    let mut result = String::new();
    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            result.push_str(preceding);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}


fn remove_html_comments(content: &str) -> String {
    
    let mut result = String::with_capacity(content.len());
    let mut state = 0; 

    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        match state {
            0 => {
                if c == '<' {
                    state = 1;
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
            1 => {
                if c == '!' {
                    state = 2;
                    result.push(c);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            2 => {
                if c == '-' {
                    state = 3;
                    result.push(c);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            3 => {
                if c == '-' {
                    
                    state = 4;
                    
                    result.truncate(result.len() - 3);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            4 => {
                
                if c == '-' {
                    
                    if chars.peek() == Some(&'-') {
                        chars.next(); 
                        if chars.peek() == Some(&'>') {
                            chars.next(); 
                            state = 0; 
                        }
                    }
                }
                
            }
            _ => unreachable!(),
        }
    }

    result
}


fn remove_css_comments(content: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < content.len() {
        if i + 1 < content.len() && &content[i..i + 2] == "/*" {
            let comment_end = content[i + 2..].find("*/");
            if let Some(end) = comment_end {
                i = i + 2 + end + 2;
            } else {
                result.push_str(&content[i..]);
                break;
            }
        } else {
            result.push(content.chars().nth(i).unwrap());
            i += 1;
        }
    }

    result
}


fn remove_shell_comments(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            
            let preceding = &line[0..comment_pos];
            if preceding.contains("echo") || preceding.contains("printf") {
                result.push_str(line);
            } else {
                result.push_str(preceding);
            }
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}


fn remove_yaml_comments(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            result.push_str(preceding);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

```

### config.rs

- **Path:** src/config.rs
- **Size:** 9.29 KB
- **Type:** rs

```rs
use crate::cli::Cli;
use anyhow::{Context, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

const CONFIG_FILENAME: &str = "remix.config.json";
const DEFAULT_MAX_FILE_SIZE: u64 = 100_000; 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    
    #[serde(default = "default_format")]
    pub format: String,

    
    #[serde(default)]
    pub open_file: bool,

    
    #[serde(default = "default_output_path")]
    pub path: String,

    
    pub instruction_file_path: Option<String>,

    
    #[serde(default)]
    pub remove_comments: bool,
}

fn default_format() -> String {
    "md".to_string()
}

fn default_output_path() -> String {
    "./remix-output.md".to_string()
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: default_format(),
            open_file: false,
            path: default_output_path(),
            instruction_file_path: None,
            remove_comments: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IgnoreConfig {
    
    #[serde(default = "default_use_gitignore")]
    pub use_gitignore: bool,

    
    #[serde(default = "default_use_default_patterns")]
    pub use_default_patterns: bool,

    
    #[serde(default = "default_use_mixignore")]
    pub use_mixignore: bool,

    
    #[serde(default)]
    pub custom_patterns: Vec<String>,
}

fn default_use_gitignore() -> bool {
    true
}

fn default_use_default_patterns() -> bool {
    true
}

fn default_use_mixignore() -> bool {
    true
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            use_gitignore: default_use_gitignore(),
            use_default_patterns: default_use_default_patterns(),
            use_mixignore: default_use_mixignore(),
            custom_patterns: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    
    #[serde(default = "default_enable_security_check")]
    pub enable_security_check: bool,
}

fn default_enable_security_check() -> bool {
    true
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_security_check: default_enable_security_check(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    
    #[serde(default)]
    pub include: Vec<String>,

    
    #[serde(default)]
    pub ignore: IgnoreConfig,

    
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,

    
    #[serde(default)]
    pub compress: bool,

    
    #[serde(default)]
    pub security: SecurityConfig,

    
    #[serde(default)]
    pub output: OutputConfig,

    
    pub instruction: Option<String>,
}

fn default_max_file_size() -> u64 {
    DEFAULT_MAX_FILE_SIZE
}

impl Default for Config {
    fn default() -> Self {
        Self {
            include: Vec::new(),
            ignore: IgnoreConfig {
                use_gitignore: default_use_gitignore(),
                use_default_patterns: default_use_default_patterns(),
                use_mixignore: default_use_mixignore(),
                custom_patterns: vec![
                    "node_modules/".to_string(),
                    "package-lock.json".to_string(),
                    "**/package-lock.json".to_string(),
                    "**/node_modules/".to_string(),
                    "**/bun.lockb".to_string(),
                    "bun.lockb".to_string(),
                    "bun.lock".to_string(),
                    ".conda/".to_string(),
                    "**/.conda/".to_string(),
                    ".venv/".to_string(),
                    "**/.venv/".to_string(),
                    ".mamba/".to_string(),
                    "**/.mamba/".to_string(),
                    ".pyenv/".to_string(),
                    "**/.pyenv/".to_string(),
                    ".git/".to_string(),
                    "**/.git/".to_string(),
                    ".gitignore".to_string(),
                    "**/.gitignore".to_string(),
                    ".gitattributes".to_string(),
                    "**/.gitattributes".to_string(),
                    ".github/".to_string(),
                    "**/.github/".to_string(),
                    ".gitmodules".to_string(),
                    "**/.gitmodules".to_string(),
                    ".gitkeep".to_string(),
                    "**/.gitkeep".to_string(),
                    "target/".to_string(),
                    "**/target/".to_string(),
                    "dist/".to_string(),
                    "**/dist/".to_string(),
                    "build/".to_string(),
                    "**/build/".to_string(),
                    "**/*.log".to_string(),
                    "**/Cargo.lock".to_string(),
                    "**/.env".to_string(),
                    "**/*.exe".to_string(),
                    "**/*.o".to_string(),
                    "**/*.so".to_string(),
                    "**/*.dylib".to_string(),
                    "**/*.dll".to_string(),
                    "**/*.lib".to_string(),
                    "**/*.a".to_string(),
                    "**/*.lib".to_string(),
                ],
            },
            max_file_size: default_max_file_size(),
            compress: false,
            security: SecurityConfig::default(),
            output: OutputConfig::default(),
            instruction: None,
        }
    }
}

impl Config {
    
    pub fn merge_with_cli(&self, cli: &Cli) -> Self {
        let mut config = self.clone();

        
        if let Some(patterns) = cli.include_patterns() {
            config.include = patterns;
        }

        if let Some(patterns) = cli.ignore_patterns() {
            config.ignore.custom_patterns = patterns;
        }

        if let Some(max_size) = cli.max_file_size {
            config.max_file_size = max_size;
        }

        if cli.compress {
            config.compress = true;
        }

        if cli.skip_sensitive_check {
            config.security.enable_security_check = false;
        }

        if cli.no_gitignore {
            config.ignore.use_gitignore = false;
        }

        if cli.no_default_patterns {
            config.ignore.use_default_patterns = false;
        }

        if let Some(format) = &cli.format {
            config.output.format = format.clone();
        }

        if let Some(output_path) = &cli.output {
            config.output.path = output_path.to_string_lossy().to_string();
        }

        if cli.open {
            config.output.open_file = true;
        }

        if cli.remove_comments {
            config.output.remove_comments = true;
        }

        if let Some(instruction) = &cli.instruction {
            config.instruction = Some(instruction.clone());
        }

        if let Some(instruction_file) = &cli.instruction_file {
            config.output.instruction_file_path =
                Some(instruction_file.to_string_lossy().to_string());
        }

        config
    }
}


pub fn find_and_load_config() -> Result<Config> {
    let current_dir = std::env::current_dir()?;
    let config_path = current_dir.join(CONFIG_FILENAME);

    if config_path.exists() {
        info!("Found configuration file: {}", config_path.display());
        load_config(&config_path)
    } else {
        warn!("No configuration file found, using defaults");
        Ok(Config::default())
    }
}


pub fn load_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read config file: {}", path.display()))?;

    let config: Config = serde_json::from_str(&content)
        .context(format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}


pub fn init_config() -> Result<()> {
    let config = Config::default();
    let current_dir = std::env::current_dir()?;
    let config_path = current_dir.join(CONFIG_FILENAME);

    if config_path.exists() {
        warn!(
            "Configuration file already exists: {}",
            config_path.display()
        );
        return Ok(());
    }

    let json =
        serde_json::to_string_pretty(&config).context("Failed to serialize configuration")?;

    let mut file = fs::File::create(&config_path).context(format!(
        "Failed to create config file: {}",
        config_path.display()
    ))?;

    file.write_all(json.as_bytes()).context(format!(
        "Failed to write to config file: {}",
        config_path.display()
    ))?;

    info!("Created configuration file: {}", config_path.display());

    Ok(())
}

```

### formatter.rs

- **Path:** src/formatter.rs
- **Size:** 9.42 KB
- **Type:** rs

```rs
use crate::config::OutputConfig;
use crate::packer::PackedRepository;
use crate::utils::{format_size, open_file};
use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::security::SecurityCheckStatus;
use rtoon;

pub fn output_result(repo: &PackedRepository, config: &OutputConfig) -> Result<()> {
    let output_path = &config.path;
    let format = &config.format;

    info!("Generating output in {} format to {}", format, output_path);

    let content = match format.as_str() {
        "md" | "markdown" => format_markdown(repo),
        "json" => format_json(repo)?,
        "txt" | "text" => format_text(repo),
        "toon" => format_toon(repo)?,
        _ => {
            warn!("Unknown format '{}', defaulting to markdown", format);
            format_markdown(repo)
        }
    };

    
    let mut file = fs::File::create(output_path)
        .context(format!("Failed to create output file: {}", output_path))?;

    file.write_all(content.as_bytes())
        .context(format!("Failed to write to output file: {}", output_path))?;

    info!("Output written to {}", output_path);

    
    if config.open_file {
        info!("Opening output file");
        open_file(output_path).context(format!("Failed to open output file: {}", output_path))?;
    }

    Ok(())
}

fn format_markdown(repo: &PackedRepository) -> String {
    let mut output = String::new();

    
    if let Some(instruction) = &repo.instruction {
        output.push_str("# User Instruction\n\n");
        output.push_str(instruction);
        output.push_str("\n\n");
    }

    
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

            
            output.push_str(&format!("- **Path:** {}\n", file.relative_path));
            output.push_str(&format!("- **Size:** {}\n", format_size(file.size)));

            if !file.extension.is_empty() {
                output.push_str(&format!("- **Type:** {}\n", file.extension));
            }

            output.push_str("\n```");

            
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

pub fn format_toon(repo: &PackedRepository) -> Result<String> {
    let value = serde_json::to_value(repo).context("Failed to convert repository to JSON value")?;
    rtoon::encode_default(&value).context("Failed to encode repository to TOON")
}

fn format_text(repo: &PackedRepository) -> String {
    let mut output = String::new();

    
    if let Some(instruction) = &repo.instruction {
        output.push_str("USER INSTRUCTION:\n\n");
        output.push_str(instruction);
        output.push_str("\n\n");
    }

    
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

```

### lib.rs

- **Path:** src/lib.rs
- **Size:** 148 bytes
- **Type:** rs

```rs
pub mod cli;
pub mod comments;
pub mod config;
pub mod formatter;
pub mod packer;
pub mod remote;
pub mod scanner;
pub mod security;
pub mod utils;

```

### main.rs

- **Path:** src/main.rs
- **Size:** 3.98 KB
- **Type:** rs

```rs
mod cli;
mod comments;
mod config;
mod formatter;
mod packer;
mod remote;
mod scanner;
mod security;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use console::style;
use env_logger::Env;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.len() > 1 && args[1] == "mix" {
        
        let filtered_args: Vec<String> = args
            .into_iter()
            .enumerate()
            .filter(|(i, arg)| *i != 1 || arg != "mix")
            .map(|(_, arg)| arg)
            .collect();
        Cli::parse_from(filtered_args)
    } else {
        Cli::parse()
    };

    
    let main_spinner = ProgressBar::new_spinner();
    main_spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {wide_msg}")
            .unwrap(),
    );

    main_spinner.set_message(format!("🚀 {} starting...", style("remix").bold().green()));
    main_spinner.tick();

    
    main_spinner.set_message("Loading configuration...");
    let config = if let Some(ref config_path) = cli.config {
        config::load_config(config_path).context(format!(
            "Failed to load config from {}",
            config_path.display()
        ))?
    } else {
        config::find_and_load_config().unwrap_or_default()
    };

    
    if cli.init {
        main_spinner.set_message("Initializing configuration...");
        config::init_config()?;
        main_spinner.finish_with_message(format!(
            "{} Configuration initialized successfully",
            style("✓").bold().green()
        ));
        return Ok(());
    }

    
    let target_path = if let Some(path) = &cli.path {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?
    };

    main_spinner.set_message(format!(
        "Starting remix on {}",
        style(target_path.display()).cyan()
    ));

    info!("Starting remix on {}", target_path.display());

    
    if let Some(remote_url) = &cli.remote {
        let branch = cli
            .remote_branch
            .as_ref()
            .map_or_else(|| "main".to_string(), |s| s.clone());
        main_spinner.set_message(format!(
            "Processing remote repository: {} ({})",
            style(remote_url).cyan(),
            style(&branch).cyan()
        ));
        info!("Processing remote repository: {} ({})", remote_url, branch);

        let temp_dir = remote::clone_repository(remote_url, &branch)
            .context("Failed to clone remote repository")?;

        main_spinner.set_message("Processing repository...");
        let result = packer::pack_repository(&temp_dir, &config.merge_with_cli(&cli)).await?;

        main_spinner.set_message("Formatting output...");
        formatter::output_result(&result, &config.output)?;
    } else {
        
        main_spinner.set_message(format!(
            "Processing local repository: {}",
            style(target_path.display()).cyan()
        ));
        info!("Processing local repository: {}", target_path.display());

        main_spinner.set_message("Processing repository...");
        let result = packer::pack_repository(&target_path, &config.merge_with_cli(&cli)).await?;

        main_spinner.set_message("Formatting output...");
        formatter::output_result(&result, &config.output)?;
    }

    main_spinner.finish_with_message(format!(
        "{} Repository packing completed successfully",
        style("✓").bold().green()
    ));
    info!("Repository packing completed successfully");
    Ok(())
}

```

### packer.rs

- **Path:** src/packer.rs
- **Size:** 12.19 KB
- **Type:** rs

```rs
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
    pub security_check_status: security::SecurityCheckStatus,  
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

    
    let multi_progress = MultiProgress::new();

    
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

    
    let binary_files: Vec<String> = files
        .iter()
        .filter(|file| file.is_binary)
        .map(|file| file.relative_path.to_string_lossy().to_string())
        .collect();

    
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

    
    let progress = Arc::new(Mutex::new(process_progress));

    
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

            
            if let Ok(pb) = progress.lock() {
                pb.inc(1);
                if let Some(content) = &result {
                    pb.set_message(format!("Processed {}", content.relative_path));
                }
            }

            result
        })
        .collect();

    
    if let Ok(pb) = progress.lock() {
        pb.finish_with_message(format!("Processed {} files", file_contents.len()));
    }

    info!("Processed {} files", file_contents.len());

    
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

    
    let summary = generate_summary(&file_contents, binary_files.len());

    
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
        suspicious_files,  
        security_check_status: security_status,  
        binary_files: Some(binary_files),
    })
}

fn read_file_content(file: &FileInfo, config: &Config) -> Result<Option<FileContent>> {
    
    if file.is_binary {
        debug!("Skipping binary file: {}", file.path.display());
        return Ok(None);
    }

    
    let content = fs::read_to_string(&file.path)
        .context(format!("Failed to read file: {}", file.path.display()))?;

    
    if config.security.enable_security_check && security::check_sensitive_content(&content) {
        warn!(
            "Skipping file with sensitive content: {}",
            file.path.display()
        );
        return Ok(None);
    }

    
    let extension = file
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    
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
    
    
    

    
    let lines: Vec<&str> = content.lines().collect();

    
    if lines.len() < 10 {
        return content.to_string();
    }

    
    let mut compressed = Vec::new();
    let mut in_comment_block = false;
    let mut consecutive_empty_lines = 0;

    for line in lines {
        let trimmed = line.trim();

        
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

        
        if trimmed.is_empty() {
            consecutive_empty_lines += 1;
            if consecutive_empty_lines <= 1 {
                compressed.push(line);
            }
            continue;
        }
        consecutive_empty_lines = 0;

        
        if trimmed.starts_with("//") {
            continue;
        }

        
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

        
        if trimmed == "{" || trimmed == "}" {
            compressed.push(line);
            continue;
        }

        
        if trimmed.contains("impl") || trimmed.contains(" for ") {
            compressed.push(line);
            continue;
        }

        
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

    
    let mut directories = std::collections::HashSet::new();

    for file in files {
        let path = PathBuf::from(&file.relative_path);
        if let Some(parent) = path.parent() {
            directories.insert(parent.to_path_buf());
        }
    }

    let directory_count = directories.len();

    
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

```

### remote.rs

- **Path:** src/remote.rs
- **Size:** 7.35 KB
- **Type:** rs

```rs
use anyhow::{Context, Result};
use git2::{BranchType, Oid, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use regex::Regex;
use std::path::PathBuf;
use tempfile::TempDir;








pub fn clone_repository(url: &str, branch: &str) -> Result<PathBuf> {
    info!("Cloning repository: {}", url);

    
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {prefix:.bold.dim} {msg}")
            .unwrap(),
    );
    progress.set_prefix("[Clone]");
    progress.set_message(format!("Parsing URL: {}", url));

    
    let (repo_url, target_ref) = parse_git_url(url, branch)?;

    
    progress.set_message("Creating temporary directory...");
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path().to_path_buf();

    debug!("Cloning to temporary directory: {}", temp_path.display());

    
    progress.set_message(format!("Cloning repository: {}...", repo_url));

    
    
    let clone_ticker = std::thread::spawn(move || {
        let pb = progress;
        let messages = [
            "Downloading objects...",
            "Resolving deltas...",
            "Checking out files...",
            "Indexing repository...",
        ];
        let mut idx = 0;
        loop {
            pb.set_message(format!("Cloning: {}", messages[idx % messages.len()]));
            idx += 1;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    
    let clone_result = match Repository::clone(&repo_url, &temp_path) {
        Ok(repo) => {
            
            if !target_ref.is_empty() && target_ref != "main" && target_ref != "master" {
                checkout_target(&repo, &target_ref)?;
            }
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(
            "Failed to clone repository: {}: {}",
            repo_url,
            e
        )),
    };

    
    clone_ticker.thread().unpark();

    
    match clone_result {
        Ok(_) => {
            
            
            std::mem::forget(temp_dir);

            info!("Repository cloned successfully to {}", temp_path.display());
            Ok(temp_path)
        }
        Err(e) => {
            
            Err(e)
        }
    }
}


fn checkout_target(repo: &Repository, target_ref: &str) -> Result<()> {
    
    if let Ok(oid) = Oid::from_str(target_ref) {
        debug!("Checking out commit: {}", target_ref);
        let commit = repo
            .find_commit(oid)
            .context(format!("Failed to find commit: {}", target_ref))?;

        
        repo.set_head_detached(commit.id())
            .context(format!("Failed to set HEAD to commit: {}", target_ref))?;

        
        let obj = repo
            .find_object(commit.id(), None)
            .context(format!("Failed to find object for commit: {}", target_ref))?;

        repo.reset(&obj, git2::ResetType::Hard, None)
            .context(format!("Failed to reset to commit: {}", target_ref))?;
    } else {
        
        debug!("Checking out branch: {}", target_ref);

        
        if let Ok(branch) = repo.find_branch(target_ref, BranchType::Local) {
            repo.set_head(branch.get().name().unwrap_or(""))
                .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
        } else {
            
            let remote_branch_name = format!("origin/{}", target_ref);
            if let Ok(branch) = repo.find_branch(&remote_branch_name, BranchType::Remote) {
                
                let _branch_ref = branch.get().name().unwrap_or("");
                repo.branch(
                    target_ref,
                    &repo.find_commit(branch.get().peel_to_commit()?.id())?,
                    false,
                )
                .context(format!(
                    "Failed to create local branch from remote: {}",
                    target_ref
                ))?;

                
                repo.set_head(&format!("refs/heads/{}", target_ref))
                    .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
            } else {
                
                return Err(anyhow::anyhow!(
                    "Could not find branch {} in repository",
                    target_ref
                ));
            }

            
            let obj = repo
                .revparse_single(&format!("refs/heads/{}", target_ref))
                .context(format!(
                    "Failed to find reference for branch: {}",
                    target_ref
                ))?;

            repo.reset(&obj, git2::ResetType::Hard, None)
                .context(format!("Failed to reset to branch: {}", target_ref))?;
        }
    }

    Ok(())
}


fn parse_git_url(url: &str, branch: &str) -> Result<(String, String)> {
    
    if !url.contains("://") && url.contains('/') {
        let repo_url = format!("https://github.com/{}", url);
        return Ok((repo_url, branch.to_string()));
    }

    
    let branch_regex = Regex::new(r"(?:https?://[^/]+/[^/]+/[^/]+)/tree/([^/]+)").unwrap();
    if let Some(captures) = branch_regex.captures(url) {
        let base_url = branch_regex.replace(url, "$1").to_string();
        let branch_name = captures.get(1).unwrap().as_str();
        return Ok((base_url, branch_name.to_string()));
    }

    
    let commit_regex = Regex::new(r"(?:https?://[^/]+/[^/]+/[^/]+)/commit/([^/]+)").unwrap();
    if let Some(captures) = commit_regex.captures(url) {
        let base_url = commit_regex.replace(url, "$1").to_string();
        let commit_hash = captures.get(1).unwrap().as_str();
        return Ok((base_url, commit_hash.to_string()));
    }

    
    Ok((url.to_string(), branch.to_string()))
}

```

### scanner.rs

- **Path:** src/scanner.rs
- **Size:** 11.08 KB
- **Type:** rs

```rs
use crate::config::Config;
use anyhow::{Context, Result};
use glob::{glob_with, MatchOptions};
use ignore::{
    gitignore::{Gitignore, GitignoreBuilder},
    WalkBuilder,
};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info, warn};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tree_magic_mini as tree_magic;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub size: u64,
    pub mime_type: String,
    pub is_binary: bool,
}

impl FileInfo {
    pub fn new(path: PathBuf, base_path: &Path) -> Result<Self> {
        let metadata = fs::metadata(&path)
            .context(format!("Failed to get metadata for {}", path.display()))?;

        let size = metadata.len();

        
        let mime_type = tree_magic::from_filepath(&path).unwrap_or("application/octet-stream");

        
        let is_binary = mime_type.starts_with("application/")
            && !mime_type.contains("json")
            && !mime_type.contains("xml")
            && !mime_type.contains("javascript")
            && !mime_type.contains("typescript")
            || mime_type.starts_with("image/")
            || mime_type.starts_with("audio/")
            || mime_type.starts_with("video/");

        let relative_path = path.strip_prefix(base_path).unwrap_or(&path).to_path_buf();

        Ok(Self {
            path,
            relative_path,
            size,
            mime_type: mime_type.to_string(),
            is_binary,
        })
    }
}


fn read_mixignore(base_path: &Path) -> Option<Gitignore> {
    let mixignore_path = base_path.join(".remixignore");
    if mixignore_path.exists() {
        debug!("Found .remixignore file");
        let mut builder = GitignoreBuilder::new(base_path);
        if builder
            .add_line(
                None,
                &fs::read_to_string(mixignore_path).unwrap_or_default(),
            )
            .is_ok()
        {
            return builder.build().ok();
        }
    }
    None
}


fn get_default_ignore_patterns() -> Vec<&'static str> {
    vec![
        "node_modules/",
        ".git/",
        ".gitignore",
        ".gitattributes",
        ".github/",
        ".gitmodules",
        ".gitkeep",
        "target/",
        "dist/",
        "build/",
        "**/*.log",
        "**/Cargo.lock",
        "**/.env",
        "**/*.exe",
        "**/*.o",
        "**/*.so",
        "**/*.dll",
        "**/*.dylib",
        "**/*.zip",
        "**/*.tar",
        "**/*.gz",
        "**/*.rar",
        "**/*.7z",
        "**/*.jar",
        "**/*.class",
        "**/*.pyc",
        "**/__pycache__/",
        "**/.idea/",
        "**/.vscode/",
        "**/node_modules/",
        "**/vendor/",
        "**/bin/",
        "**/obj/",
        "**/build/",
    ]
}

/// Normalize a path string for glob matching
fn normalize_path(path: &str) -> String {
    // Convert Windows backslashes to forward slashes for consistent glob matching
    path.replace('\\', "/")
}

/// Check if a path should be ignored based on common patterns
/// Used both by scanner and security check
pub fn should_ignore_common(path: &Path) -> bool {
    let path_str = normalize_path(&path.to_string_lossy());

    // Common directories that should always be ignored
    if path_str.contains("/target/")
        || path_str.starts_with("target/")
        || path_str.contains("/.git/")
        || path_str.starts_with(".git/")
        || path_str.contains("/node_modules/")
        || path_str.starts_with("node_modules/")
        || path_str.contains("/dist/")
        || path_str.starts_with("dist/")
        || path_str.contains("/build/")
        || path_str.starts_with("build/")
    {
        return true;
    }

    // Common files that should always be ignored
    if path_str.ends_with(".exe")
        || path_str.ends_with(".o")
        || path_str.ends_with(".obj")
        || path_str.ends_with(".dll")
        || path_str.ends_with(".so")
        || path_str.ends_with(".dylib")
        || path_str.ends_with(".class")
        || path_str.ends_with(".jar")
        || path_str.ends_with(".war")
        || path_str.ends_with(".zip")
        || path_str.ends_with(".tar")
        || path_str.ends_with(".gz")
        || path_str.ends_with(".rar")
        || path_str.ends_with(".7z")
        || path_str.ends_with(".pyc")
    {
        return true;
    }

    false
}

pub fn scan_repository(base_path: &Path, config: &Config) -> Result<Vec<FileInfo>> {
    info!("Scanning repository at {}", base_path.display());

    // Create a progress bar for scanning
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Scanning repository files...");

    // Initialize the ignore system with the appropriate layers
    let should_ignore = |path: &Path| -> bool {
        // First check common patterns that should always be ignored
        if should_ignore_common(path) {
            return true;
        }

        let relative_path = path.strip_prefix(base_path).unwrap_or(path);
        let path_str = normalize_path(&relative_path.to_string_lossy());

        // Layer 1 (highest priority): Custom ignore patterns
        if !config.ignore.custom_patterns.is_empty() {
            for pattern in &config.ignore.custom_patterns {
                // Normalize the pattern too
                let pattern = normalize_path(pattern);

                if glob_match::glob_match(&pattern, &path_str) {
                    debug!(
                        "Ignoring '{}' due to custom pattern '{}'",
                        path_str, pattern
                    );
                    return true;
                }
            }
        }

        // Layer 2: .remixignore file
        if config.ignore.use_mixignore {
            if let Some(ref mixignore) = read_mixignore(base_path) {
                let result = mixignore.matched_path_or_any_parents(relative_path, false);
                if result.is_ignore() {
                    debug!("Ignoring '{}' due to .remixignore", path_str);
                    return true;
                }
            }
        }

        // Layer 3: Default ignore patterns if enabled
        if config.ignore.use_default_patterns {
            for pattern in get_default_ignore_patterns() {
                if glob_match::glob_match(pattern, &path_str) {
                    debug!(
                        "Ignoring '{}' due to default pattern '{}'",
                        path_str, pattern
                    );
                    return true;
                }
            }
        }

        false
    };

    // Use the ignore crate to build a walker that respects .gitignore if enabled
    let mut walker = WalkBuilder::new(base_path);
    walker.hidden(false); // Include hidden files/directories

    // Add standard directories to always ignore
    walker.filter_entry(|entry| {
        let path = entry.path();

        // Skip target/ and .git/ directories completely
        if (path.ends_with("target") || path.ends_with(".git"))
            && entry.file_type().is_some_and(|ft| ft.is_dir())
        {
            debug!("Ignoring directory: {}", path.display());
            return false;
        }

        true
    });

    if config.ignore.use_gitignore {
        walker.git_ignore(true);
        walker.git_global(true);
        walker.git_exclude(true);
    } else {
        walker.git_ignore(false);
        walker.git_global(false);
        walker.git_exclude(false);
    }

    // Filter files using our multi-layered ignore system
    spinner.set_message("Collecting files...");
    let mut files: Vec<PathBuf> = walker
        .build()
        .filter_map(|entry| {
            spinner.tick();
            entry.ok()
        })
        .filter(|entry| {
            let path = entry.path();

            // Skip if not a regular file
            if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                return false;
            }

            // Use our custom ignore function
            !should_ignore(path)
        })
        .map(|entry| entry.into_path())
        .collect();

    // Apply include patterns if specified
    if !config.include.is_empty() {
        spinner.set_message(format!("Applying include patterns: {:?}", config.include));

        let mut included_files = HashSet::new();
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        for pattern in &config.include {
            // Normalize pattern for consistent matching
            let normalized_pattern = normalize_path(pattern);
            let full_pattern = base_path.join(&normalized_pattern);
            let pattern_str = full_pattern.to_string_lossy().to_string();

            match glob_with(&pattern_str, options) {
                Ok(entries) => {
                    for entry in entries.filter_map(Result::ok) {
                        included_files.insert(entry);
                    }
                }
                Err(e) => {
                    warn!("Invalid include pattern '{}': {}", pattern, e);
                }
            }
        }

        // Only keep files that match the include patterns
        if !included_files.is_empty() {
            files.retain(|path| included_files.contains(path));
        }
    }

    spinner.set_message("Processing file information...");

    // Process file information in parallel
    let file_infos: Vec<FileInfo> = files
        .par_iter()
        .filter_map(|path| {
            match FileInfo::new(path.clone(), base_path) {
                Ok(info) => {
                    // Filter out files larger than the max size
                    if info.size > config.max_file_size {
                        debug!(
                            "Skipping large file: {} ({} bytes)",
                            info.path.display(),
                            info.size
                        );
                        None
                    } else if info.is_binary
                        && !config
                            .include
                            .iter()
                            .any(|p| p.contains("*.bin") || p.contains("binary"))
                    {
                        debug!(
                            "Skipping binary file: {} ({})",
                            info.path.display(),
                            info.mime_type
                        );
                        None
                    } else {
                        Some(info)
                    }
                }
                Err(e) => {
                    warn!("Error processing file {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    spinner.finish_with_message(format!("Found {} files to process", file_infos.len()));
    info!("Found {} files to process", file_infos.len());

    Ok(file_infos)
}

```

### utils.rs

- **Path:** src/utils.rs
- **Size:** 1019 bytes
- **Type:** rs

```rs

pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}


pub fn open_file(path: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "start", "", path])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open").arg(path).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}

```

## tests/

### cli_tests.rs

- **Path:** tests/cli_tests.rs
- **Size:** 1.40 KB
- **Type:** rs

```rs
use remix::cli::Cli;
use clap::Parser;

#[test]
fn test_cli_parsing() {
    
    let cli = Cli::parse_from(["remix", "path/to/repo"]);
    assert_eq!(cli.path, Some("path/to/repo".to_string()));
    assert!(cli.config.is_none());
    assert!(!cli.init);
    assert!(cli.include.is_none());
    assert!(cli.ignore.is_none());
    assert!(!cli.compress);
    assert!(!cli.skip_sensitive_check);
    assert!(!cli.remove_comments);
}

#[test]
fn test_cli_include_ignore_patterns() {
    
    let cli = Cli::parse_from([
        "remix",
        "--include",
        "*.rs,*.md",
        "--ignore",
        "target/,node_modules/",
    ]);

    let include_patterns = cli.include_patterns().unwrap();
    let ignore_patterns = cli.ignore_patterns().unwrap();

    assert_eq!(include_patterns.len(), 2);
    assert_eq!(include_patterns[0], "*.rs");
    assert_eq!(include_patterns[1], "*.md");

    assert_eq!(ignore_patterns.len(), 2);
    assert_eq!(ignore_patterns[0], "target/");
    assert_eq!(ignore_patterns[1], "node_modules/");
}

#[test]
fn test_cli_remote_repo() {
    
    let cli = Cli::parse_from([
        "remix",
        "--remote",
        "username/repo",
        "--remote-branch",
        "main",
    ]);

    assert_eq!(cli.remote, Some("username/repo".to_string()));
    assert_eq!(cli.remote_branch, Some("main".to_string()));
}

```

### config_tests.rs

- **Path:** tests/config_tests.rs
- **Size:** 804 bytes
- **Type:** rs

```rs
use remix::config::{Config, OutputConfig};

#[test]
fn test_default_config() {
    let config = Config::default();

    
    assert!(config.include.is_empty());
    assert_eq!(config.max_file_size, 100_000); 
    assert!(!config.compress);
    assert!(config.security.enable_security_check);
    assert_eq!(config.output.format, "md");
    assert!(!config.output.open_file);
    assert_eq!(config.output.path, "./remix-output.md");
}

#[test]
fn test_output_config_default() {
    let output_config = OutputConfig::default();

    assert_eq!(output_config.format, "md");
    assert!(!output_config.open_file);
    assert_eq!(output_config.path, "./remix-output.md");
    assert!(output_config.instruction_file_path.is_none());
    assert!(!output_config.remove_comments);
}

```

### packer_tests.rs

- **Path:** tests/packer_tests.rs
- **Size:** 2.01 KB
- **Type:** rs

```rs
use remix::config::Config;
use remix::packer::pack_repository;


mod common;

#[tokio::test]
async fn test_pack_repository_basic() {
    
    let test_dir = common::create_test_repo();

    
    let config = Config::default();

    
    let result = pack_repository(test_dir.path(), &config)
        .await
        .expect("Failed to pack repository");

    
    assert!(!result.files.is_empty(), "No files were packed");
    assert!(result.summary.file_count > 0, "File count should be > 0");
    assert!(result.summary.total_size > 0, "Total size should be > 0");

    
    println!("File count: {}", result.summary.file_count);
    println!("Directory count: {}", result.summary.directory_count);
    println!(
        "Files found: {:?}",
        result
            .files
            .iter()
            .map(|f| &f.relative_path)
            .collect::<Vec<_>>()
    );

    
    let file_paths: Vec<String> = result
        .files
        .iter()
        .map(|f| f.relative_path.replace('\\', "/"))
        .collect();

    
    let has_readme = file_paths.iter().any(|path| path.contains("README.md"));
    let has_src_file = file_paths.iter().any(|path| path.contains("src/"));

    assert!(
        has_readme,
        "README.md should be included in the packed files"
    );
    assert!(
        has_src_file,
        "At least one file from src/ should be included"
    );

    
    let has_node_modules = file_paths.iter().any(|path| path.contains("node_modules"));
    assert!(!has_node_modules, "node_modules should be ignored");

    
    if let Some(suspicious) = &result.suspicious_files {
        println!("Suspicious files: {:?}", suspicious);
    }
}

```

### utils_tests.rs

- **Path:** tests/utils_tests.rs
- **Size:** 1.00 KB
- **Type:** rs

```rs
use remix::utils;

#[test]
fn test_format_size_bytes() {
    assert_eq!(utils::format_size(0), "0 bytes");
    assert_eq!(utils::format_size(1), "1 bytes");
    assert_eq!(utils::format_size(512), "512 bytes");
    assert_eq!(utils::format_size(1023), "1023 bytes");
}

#[test]
fn test_format_size_kilobytes() {
    assert_eq!(utils::format_size(1024), "1.00 KB");
    assert_eq!(utils::format_size(1536), "1.50 KB");
    assert_eq!(utils::format_size(2048), "2.00 KB");
    assert_eq!(utils::format_size(1024 * 1024 - 1), "1024.00 KB");
}

#[test]
fn test_format_size_megabytes() {
    assert_eq!(utils::format_size(1024 * 1024), "1.00 MB");
    assert_eq!(utils::format_size(1536 * 1024), "1.50 MB");
    assert_eq!(utils::format_size(1024 * 1024 * 1024 - 1), "1024.00 MB");
}

#[test]
fn test_format_size_gigabytes() {
    assert_eq!(utils::format_size(1024 * 1024 * 1024), "1.00 GB");
    assert_eq!(utils::format_size(1536 * 1024 * 1024), "1.50 GB");
    assert_eq!(utils::format_size(2048 * 1024 * 1024), "2.00 GB");
}

```

### formatter_tests.rs

- **Path:** tests/formatter_tests.rs
- **Size:** 1.96 KB
- **Type:** rs

```rs
use anyhow::Result;
use remix::formatter::format_toon;
use remix::packer::{PackedRepository, RepositorySummary, FileContent};

#[test]
fn test_format_toon_basic() -> Result<()> {
    let repo = PackedRepository {
        summary: RepositorySummary {
            file_count: 2,
            directory_count: 1,
            total_size: 100,
            extensions: vec!["txt".to_string(), "rs".to_string()],
            binary_file_count: 0,
        },
        files: vec![
            FileContent {
                relative_path: "file1.txt".to_string(),
                content: "Hello".to_string(),
                size: 5,
                extension: "txt".to_string(),
                is_binary: false,
            },
            FileContent {
                relative_path: "file2.rs".to_string(),
                content: "fn main() {}".to_string(),
                size: 11,
                extension: "rs".to_string(),
                is_binary: false,
            },
        ],
        instruction: Some("Test instruction".to_string()),
        suspicious_files: None,
        security_check_status: remix::security::SecurityCheckStatus::CompletedNoFindings,
        binary_files: None,
    };

    let result = format_toon(&repo)?;
    assert!(!result.is_empty());
    assert!(result.contains("files[2]"));
    assert!(result.contains("file_count: 2"));
    Ok(())
}

#[test]
fn test_format_toon_empty_repo() -> Result<()> {
    let repo = PackedRepository {
        summary: RepositorySummary {
            file_count: 0,
            directory_count: 0,
            total_size: 0,
            extensions: vec![],
            binary_file_count: 0,
        },
        files: vec![],
        instruction: None,
        suspicious_files: None,
        security_check_status: remix::security::SecurityCheckStatus::CompletedNoFindings,
        binary_files: None,
    };

    let result = format_toon(&repo)?;
    assert!(!result.is_empty());
    assert!(result.contains("file_count: 0"));
    Ok(())
}
```


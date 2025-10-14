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

<div align="center">
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
    
    style A fill:#e1f5fe
    style JJ fill:#c8e6c9
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
| 📝 **Multiple Output Formats** | Markdown, JSON, and plain text support |
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
git clone https://github.com/dotZeroSlash/remix.git
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

# Change output format (md, json, txt)
remix --format json

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

**Basic configuration with custom includes:**

```json
{
  "include": ["src/**/*.rs", "Cargo.toml", "README.md"],
  "output": {
    "path": "./my-project.md"
  }
}
```

**Configuration for a Node.js project:**

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

**Configuration for AI analysis:**

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

### Text

Plain text output with:
- Simple file headers
- Raw code content
- Minimal formatting

## 💡 Examples

<div align="center">
  <table>
    <tr>
      <td align="center">
        <b>Input Repository</b><br>
        <img src="https://via.placeholder.com/300x200?text=Repository+Structure" alt="Repository structure" width="300"/>
      </td>
      <td align="center">
        <b>Output for AI</b><br>
        <img src="https://via.placeholder.com/300x200?text=AI+Friendly+Output" alt="AI-friendly output" width="300"/>
      </td>
    </tr>
  </table>
</div>

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
  <p>Built with ❤️ by the Rust community</p>
  <p>
    <a href="https://github.com/dotZeroSlash/remix/issues">Report Bug</a> •
    <a href="https://github.com/dotZeroSlash/remix/issues">Request Feature</a>
  </p>
</div>

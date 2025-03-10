<div align="center">
  
# 🚀 Cargo Mix

[![Crates.io](https://img.shields.io/crates/v/cargo-mix)](https://crates.io/crates/cargo-mix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)

**A blazing-fast Rust implementation of [repomix](https://github.com/yamadashy/repomix)**

*Pack your entire repository into a single, AI-friendly file*

[Installation](#-installation) • 
[Usage](#-usage) • 
[Configuration](#-configuration) • 
[Features](#-features) • 
[License](#-license)

</div>

---

## 📖 Overview

Cargo Mix prepares your codebase for AI analysis by packing it into a single file. Perfect for feeding your code to Large Language Models (LLMs) like Claude, ChatGPT, DeepSeek, Perplexity, Gemini, and more.

<div align="center">
  <img src="https://via.placeholder.com/800x400?text=Cargo+Mix+Diagram" alt="Cargo Mix workflow diagram" width="600"/>
  <p><i>A visualization of how Cargo Mix processes your repository</i></p>
</div>

## ✨ Features

| Feature | Description |
|---------|-------------|
| ⚡ **High Performance** | Built in Rust for maximum speed and efficiency |
| 💻 **Local Repositories** | Pack your entire local repository or specific directories |
| 🌐 **Remote Repositories** | Directly pack GitHub repositories with branch/tag/commit support |
| 🎯 **Intelligent File Selection** | Include/exclude files using glob patterns |
| 🛡️ **Multi-layered Ignore System** | Uses `.gitignore`, `.cargo-mixignore`, and custom ignore patterns |
| 💬 **Comment Processing** | Option to remove or keep code comments to reduce token count |
| 🔒 **Security Checks** | Detection of sensitive information like API keys and credentials |
| 🎨 **Format Customization** | Control the output format for specific AI tools |

## 📦 Installation

**Via Cargo:**
```bash
cargo install cargo-mix
```

**From Source:**
```bash
git clone https://github.com/dotZeroSlash/cargo-mix
cd cargo-mix
cargo install --path .
```

## 🔧 Usage

### Basic Commands

Pack your entire repository:
```bash
cargo mix
```

Pack a specific directory:
```bash
cargo mix path/to/directory
```

### Advanced Options

<details>
<summary>📂 <b>File Selection</b></summary>

```bash
# Include specific files or directories using glob patterns
cargo mix --include "src/**/*.rs,**/*.md"

# Exclude specific files or directories
cargo mix --ignore "**/*.log,target/"
```
</details>

<details>
<summary>🌐 <b>Remote Repositories</b></summary>

```bash
# Pack a remote repository (full URL)
cargo mix --remote https://github.com/username/repo

# Using GitHub shorthand
cargo mix --remote username/repo

# Specify branch, tag, or commit hash
cargo mix --remote username/repo --remote-branch main
cargo mix --remote https://github.com/username/repo/tree/branch-name
cargo mix --remote https://github.com/username/repo/commit/commit-hash
```
</details>

<details>
<summary>⚙️ <b>Processing Options</b></summary>

```bash
# Process comments in code
cargo mix --remove-comments

# Control security checks
cargo mix --disable-security-checks

# Compress the output
cargo mix --compress

# Initialize a new configuration file
cargo mix --init
```
</details>

## 📝 Configuration

Create a `cargo-mix.config.json` file in your project root for custom configurations:

```json
{
  "include": ["**/*.rs", "**/*.md"],
  "ignore": ["target/", "node_modules/", "**/*.log"],
  "maxFileSize": 1000000,
  "compress": true,
  "removeComments": false,
  "enableSecurityChecks": true,
  "output": {
    "format": "markdown",
    "openFile": true,
    "path": "./output.md"
  }
}
```

## 🚫 Ignore Files

Cargo Mix uses a multi-layered ignore system:

1. **`.gitignore`** - Standard Git ignore patterns are respected
2. **`.cargo-mixignore`** - Project-specific ignore patterns for Cargo Mix
3. **Command line `--ignore` patterns** or configuration file settings

A default `.cargo-mixignore` file contains sensible defaults for common files and directories that should be excluded from AI processing.

## 📊 Examples

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
    <a href="https://github.com/dotZeroSlash/cargo-mix/issues">Report Bug</a> •
    <a href="https://github.com/dotZeroSlash/cargo-mix/issues">Request Feature</a>
  </p>
</div> 
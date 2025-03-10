# Cargo Mix

A blazing-fast Rust implementation of [repomix](https://github.com/yamadashy/repomix), designed to pack your entire repository into a single, AI-friendly file. Perfect for when you need to feed your codebase to Large Language Models (LLMs) like Claude, ChatGPT, DeepSeek, Perplexity, Gemini, and more.

## Features

- **High Performance**: Built in Rust for maximum speed and efficiency
- **Local Repositories**: Pack your entire local repository or specific directories
- **Remote Repositories**: Directly pack GitHub repositories with branch/tag/commit support
- **Intelligent File Selection**: Include/exclude files using glob patterns
- **Multi-layered Ignore System**: Uses `.gitignore`, `.cargo-mixignore`, and custom ignore patterns
- **Comment Processing**: Option to remove or keep code comments to reduce token count
- **Security Checks**: Detection of sensitive information like API keys and credentials
- **Format Customization**: Control the output format for specific AI tools

## Installation

```bash
cargo install cargo-mix
```

Or install from source:

```bash
git clone https://github.com/dotZeroSlash/cargo-mix
cd cargo-mix
cargo install --path .
```

## Usage

Pack your entire repository:

```bash
cargo mix
```

Pack a specific directory:

```bash
cargo mix path/to/directory
```

Include specific files or directories using glob patterns:

```bash
cargo mix --include "src/**/*.rs,**/*.md"
```

Exclude specific files or directories:

```bash
cargo mix --ignore "**/*.log,target/"
```

Pack a remote repository:

```bash
cargo mix --remote https://github.com/username/repo
```

You can also use GitHub shorthand:

```bash
cargo mix --remote username/repo
```

Specify branch, tag, or commit hash:

```bash
cargo mix --remote username/repo --remote-branch main
cargo mix --remote https://github.com/username/repo/tree/branch-name
cargo mix --remote https://github.com/username/repo/commit/commit-hash
```

Process comments in code:

```bash
cargo mix --remove-comments
```

Control security checks:

```bash
cargo mix --disable-security-checks
```

Compress the output:

```bash
cargo mix --compress
```

Initialize a new configuration file:

```bash
cargo mix --init
```

## Configuration

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

## Ignore Files

Cargo Mix uses a multi-layered ignore system:

1. `.gitignore` - Standard Git ignore patterns are respected
2. `.cargo-mixignore` - Project-specific ignore patterns for Cargo Mix
3. Command line `--ignore` patterns or configuration file settings

A default `.cargo-mixignore` file contains sensible defaults for common files and directories that should be excluded from AI processing.

## License

MIT 
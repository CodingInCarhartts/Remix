# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-01

### Added
- **TOON Output Format**: New token-efficient output format using the `rtoon` library
  - Reduces token count by 2-5% for code repositories compared to JSON
  - Up to 30-60% savings for structured data
  - Use with `--format toon` option
- Comprehensive test suite with 34 tests covering CLI, config, scanning, formatting, and security
- Integration testing with real repositories (serde crate) validating TOON efficiency

### Changed
- Updated dependencies: Added `rtoon = "0.1.3"`
- Made formatter functions public for testing
- Enhanced CLI argument validation for output formats

### Fixed
- Improved scanner logic for include pattern matching when using subdirectories

## [0.1.0] - 2025-10-XX

### Added
- Initial release of Remix - Rust implementation of repomix
- Repository packing for AI tools
- Multiple output formats: Markdown, JSON, Text
- Remote repository support
- Intelligent file filtering with include/exclude patterns
- Multi-layered ignore system (.gitignore, .mixignore, custom patterns)
- Security checks for sensitive information
- Comment removal feature
- Flexible JSON configuration
- High-performance parallel processing
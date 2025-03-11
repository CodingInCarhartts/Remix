# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-03-11

### Added
- Initial release of cargo-mix
- Core functionality:
  - High-performance Rust implementation of repository packing for AI tools
  - Support for local repositories with directory-specific packing
  - Remote GitHub repository packing with branch/tag/commit support
  - Intelligent file selection with glob pattern inclusion/exclusion
  - Multi-layered ignore system (.gitignore, .cargo-mixignore, custom patterns)
  - Comment processing to optionally remove code comments
  - Security checks to detect sensitive information
  - Format customization for different AI tools
  - Configuration system with JSON support
- Development:
  - CI/CD pipeline with GitHub Actions for automated testing and releases
  - Comprehensive test suite for core functionality
  - CHANGELOG.md to track version changes

[Unreleased]: https://github.com/dotZeroSlash/cargo-mix/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/dotZeroSlash/cargo-mix/releases/tag/v0.1.0 
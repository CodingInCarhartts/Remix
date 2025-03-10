use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Create a temporary directory with test files for testing
pub fn create_test_repo() -> TempDir {
    let dir = tempfile::tempdir().expect("Failed to create temporary directory");
    
    // Create a basic repository structure for testing
    fs::create_dir(dir.path().join("src")).expect("Failed to create src directory");
    fs::create_dir(dir.path().join("docs")).expect("Failed to create docs directory");
    fs::create_dir_all(dir.path().join("node_modules/test-pkg")).expect("Failed to create node_modules directory");
    
    // Create a sample Rust file
    fs::write(
        dir.path().join("src/main.rs"),
        r#"fn main() {
    println!("Hello, cargo-mix!");
}
"#
    ).expect("Failed to write main.rs");
    
    // Create a sample README file
    fs::write(
        dir.path().join("README.md"),
        r#"# Test Repository

This is a test repository for cargo-mix testing.

## Features

- Feature 1
- Feature 2
"#
    ).expect("Failed to write README.md");
    
    // Create a gitignore file
    fs::write(
        dir.path().join(".gitignore"),
        r#"target/
node_modules/
*.log
"#
    ).expect("Failed to write .gitignore");
    
    // Create a sensitive file for security tests
    fs::write(
        dir.path().join("config.js"),
        r#"const API_KEY = "sk_test_1234567890";
const DB_PASSWORD = "postgres_password";

module.exports = {
    apiKey: API_KEY,
    dbConfig: {
        username: "admin",
        password: DB_PASSWORD
    }
};
"#
    ).expect("Failed to write config.js");
    
    // Create a file that should be ignored
    fs::write(
        dir.path().join("node_modules/test-pkg/package.json"),
        r#"{
  "name": "test-pkg",
  "version": "1.0.0"
}
"#
    ).expect("Failed to write package.json");
    
    dir
}

/// Get a relative path string from a base directory and a path
pub fn get_relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
} 
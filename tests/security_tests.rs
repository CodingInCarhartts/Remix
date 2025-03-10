use cargo_mix::security;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_check_sensitive_content() {
    // Test strings with sensitive information based on actual keywords
    let sensitive_content = [
        "const API_KEY = 'abc123'",
        "const JWT_SECRET = 'secret123'",
        "AWS_SECRET_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE",
        "const API_TOKEN = 'abc123'",
        "mongodb://username:password@localhost:27017",
        "postgres://user:password@localhost:5432/mydb",
    ];
    
    // Test strings without sensitive information
    let safe_content = vec![
        "const NAME = 'John Doe'",
        "const VERSION = '1.0.0'",
        "import { useState } from 'react'",
        "log.info('Application started')",
        "const mock_data = { test: true }"
    ];
    
    // Check for at least one positive match - we need to verify the function works
    let some_sensitive_detected = sensitive_content.iter()
        .any(|content| security::check_sensitive_content(content));
    
    assert!(some_sensitive_detected, 
            "Security check should detect at least one sensitive content example");
    
    // Verify non-detection of safe content
    for content in safe_content {
        assert!(!security::check_sensitive_content(content),
                "Incorrectly flagged safe content as sensitive: {}", content);
    }
}

#[test]
fn test_perform_security_check() {
    // Create a test directory with some sensitive files
    let dir = tempdir().expect("Failed to create temporary directory");
    
    // File with API key (should be detected)
    let api_file_path = dir.path().join("api.js");
    fs::write(
        &api_file_path,
        "const API_KEY = 'abc123';\nconst API_SECRET = 'secret456';\n"
    ).expect("Failed to write sensitive file");
    
    // File with database connection string (should be detected)
    let db_file_path = dir.path().join("db.js");
    fs::write(
        &db_file_path,
        "const DB_URI = 'postgres://user:password@localhost:5432/mydb';\n"
    ).expect("Failed to write sensitive db file");
    
    // Regular file (should not be detected)
    let safe_file_path = dir.path().join("safe.js");
    fs::write(
        &safe_file_path,
        "const VERSION = '1.0.0';\nconsole.log('App version:', VERSION);\n"
    ).expect("Failed to write safe file");
    
    // Perform security check
    let suspicious_files = security::perform_security_check(dir.path())
        .expect("Security check failed");
    
    // Verify that security check works in general - should detect at least one file
    assert!(!suspicious_files.is_empty(), 
            "Security check should detect at least one suspicious file");
} 
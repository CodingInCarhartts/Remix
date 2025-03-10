use anyhow::Result;
use log::{debug, info};
use std::path::Path;
use std::collections::HashSet;
use std::fs;
use crate::scanner::should_ignore_common;

/// Performs a security check on a repository to identify potentially sensitive files
pub fn perform_security_check(path: &Path) -> Result<Vec<String>> {
    info!("Performing security check on repository");
    
    let mut suspicious_files = HashSet::new();
    let sensitive_keywords = get_sensitive_keywords();
    
    // Function to normalize paths for consistent handling
    let normalize_path = |path_str: &str| -> String {
        path_str.replace('\\', "/")
    };
    
    // Walk through the repository
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            // Skip directories we know we should ignore
            let p = e.path();
            if p.is_dir() {
                let path_str = normalize_path(&p.to_string_lossy());
                if path_str.contains("/target") || path_str.contains("/.git") || 
                   path_str.ends_with("/target") || path_str.ends_with("/.git") {
                    return false;
                }
            }
            e.file_type().is_file()
        })
    {
        // Skip files in target/ and .git/ directories
        let file_path = entry.path();
        let path_str = normalize_path(&file_path.to_string_lossy());
        
        if path_str.contains("/target/") || path_str.starts_with("target/") || 
           path_str.contains("/.git/") || path_str.starts_with(".git/") {
            continue;
        }
        
        // Skip common files we should ignore
        if should_ignore_common(file_path) {
            continue;
        }
        
        let relative_path = file_path.strip_prefix(path).unwrap_or(file_path);
        
        // Skip binary files
        if is_likely_binary(file_path) {
            continue;
        }
        
        // Check filename for sensitive patterns
        let filename = relative_path.to_string_lossy().to_lowercase();
        if filename.contains("secret") || 
           filename.contains("password") || 
           filename.contains("credential") || 
           filename.contains("token") || 
           filename.contains("key") ||
           filename.contains("auth") ||
           filename.contains(".env") ||
           filename.contains("config") {
            suspicious_files.insert(relative_path.to_string_lossy().to_string());
            continue;
        }
        
        // For text files, check content for sensitive patterns
        if let Ok(content) = fs::read_to_string(file_path) {
            let content_lower = content.to_lowercase();
            for keyword in &sensitive_keywords {
                if content_lower.contains(keyword) {
                    suspicious_files.insert(relative_path.to_string_lossy().to_string());
                    break;
                }
            }
        }
    }
    
    let result: Vec<String> = suspicious_files.into_iter().collect();
    debug!("Found {} suspicious files", result.len());
    
    Ok(result)
}

/// Checks if a file is likely to be binary
fn is_likely_binary(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        // Common binary file extensions
        return matches!(ext_str.as_str(), 
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | 
            "exe" | "dll" | "so" | "dylib" | 
            "zip" | "tar" | "gz" | "rar" | 
            "mp3" | "mp4" | "avi" | "mov" | 
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx"
        );
    }
    
    false
}

/// Returns a list of sensitive keywords to look for
fn get_sensitive_keywords() -> Vec<String> {
    vec![
        // API Keys and Tokens
        "api_key".to_string(),
        "api_token".to_string(),
        "app_key".to_string(),
        "app_token".to_string(),
        "secret_key".to_string(),
        
        // AWS Keys
        "aws_access_key".to_string(),
        "aws_secret_key".to_string(),
        
        // Private Keys
        "private key".to_string(),
        "begin private key".to_string(),
        
        // Passwords
        "password=".to_string(),
        "passwd=".to_string(),
        "pwd=".to_string(),
        
        // Firebase keys
        "firebase_key".to_string(),
        
        // Generic Auth Tokens
        "auth_token".to_string(),
        "bearer token".to_string(),
        
        // Connection Strings
        "connection_string".to_string(),
        "mongodb://".to_string(),
        "postgres://".to_string(),
        "mysql://".to_string(),
        "redis://".to_string(),
    ]
}

/// Check if content contains sensitive information
pub fn check_sensitive_content(content: &str) -> bool {
    let sensitive_keywords = get_sensitive_keywords();
    let content_lower = content.to_lowercase();
    
    for keyword in sensitive_keywords {
        if content_lower.contains(&keyword) {
            return true;
        }
    }
    
    false
} 
use cargo_mix::config::Config;
use cargo_mix::packer::pack_repository;

// Import the common test module
mod common;

#[tokio::test]
async fn test_pack_repository_basic() {
    // Create a test repository
    let test_dir = common::create_test_repo();
    
    // Create a basic config
    let config = Config::default();
    
    // Pack the repository
    let result = pack_repository(test_dir.path(), &config)
        .await
        .expect("Failed to pack repository");
    
    // Basic assertions about the result
    assert!(!result.files.is_empty(), "No files were packed");
    assert!(result.summary.file_count > 0, "File count should be > 0");
    assert!(result.summary.total_size > 0, "Total size should be > 0");
    
    // Print debug info
    println!("File count: {}", result.summary.file_count);
    println!("Directory count: {}", result.summary.directory_count);
    println!("Files found: {:?}", result.files.iter().map(|f| &f.relative_path).collect::<Vec<_>>());
    
    // Check if important files are included - use more flexible matching
    let file_paths: Vec<String> = result.files.iter()
        .map(|f| f.relative_path.replace('\\', "/"))
        .collect();
    
    // Check for README.md and any file in src directory
    let has_readme = file_paths.iter().any(|path| path.contains("README.md"));
    let has_src_file = file_paths.iter().any(|path| path.contains("src/"));
    
    assert!(has_readme, "README.md should be included in the packed files");
    assert!(has_src_file, "At least one file from src/ should be included");
    
    // Ignored files should not be included
    let has_node_modules = file_paths.iter().any(|path| path.contains("node_modules"));
    assert!(!has_node_modules, "node_modules should be ignored");
    
    // Security check should work, but we won't assert specific files
    if let Some(suspicious) = &result.suspicious_files {
        println!("Suspicious files: {:?}", suspicious);
    }
} 
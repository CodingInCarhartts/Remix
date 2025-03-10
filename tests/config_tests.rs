use cargo_mix::config::{Config, OutputConfig};
use std::path::PathBuf;

#[test]
fn test_default_config() {
    let config = Config::default();
    
    // Test default values
    assert!(config.include.is_empty());
    assert_eq!(config.max_file_size, 100_000); // 100KB
    assert!(!config.compress);
    assert!(config.security.enable_security_check);
    assert_eq!(config.output.format, "md");
    assert!(!config.output.open_file);
    assert_eq!(config.output.path, "./cargo-mix-output.md");
}

#[test]
fn test_output_config_default() {
    let output_config = OutputConfig::default();
    
    assert_eq!(output_config.format, "md");
    assert!(!output_config.open_file);
    assert_eq!(output_config.path, "./cargo-mix-output.md");
    assert!(output_config.instruction_file_path.is_none());
    assert!(!output_config.remove_comments);
} 
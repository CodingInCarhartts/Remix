use cargo_mix::cli::Cli;
use clap::Parser;

#[test]
fn test_cli_parsing() {
    // Test basic CLI parsing
    let cli = Cli::parse_from(["remix", "path/to/repo"]);
    assert_eq!(cli.path, Some("path/to/repo".to_string()));
    assert!(cli.config.is_none());
    assert!(!cli.init);
    assert!(cli.include.is_none());
    assert!(cli.ignore.is_none());
    assert!(!cli.compress);
    assert!(!cli.skip_sensitive_check);
    assert!(!cli.remove_comments);
}

#[test]
fn test_cli_include_ignore_patterns() {
    // Test include patterns parsing
    let cli = Cli::parse_from([
        "remix",
        "--include",
        "*.rs,*.md",
        "--ignore",
        "target/,node_modules/",
    ]);

    let include_patterns = cli.include_patterns().unwrap();
    let ignore_patterns = cli.ignore_patterns().unwrap();

    assert_eq!(include_patterns.len(), 2);
    assert_eq!(include_patterns[0], "*.rs");
    assert_eq!(include_patterns[1], "*.md");

    assert_eq!(ignore_patterns.len(), 2);
    assert_eq!(ignore_patterns[0], "target/");
    assert_eq!(ignore_patterns[1], "node_modules/");
}

#[test]
fn test_cli_remote_repo() {
    // Test remote repository options
    let cli = Cli::parse_from([
        "remix",
        "--remote",
        "username/repo",
        "--remote-branch",
        "main",
    ]);

    assert_eq!(cli.remote, Some("username/repo".to_string()));
    assert_eq!(cli.remote_branch, Some("main".to_string()));
}

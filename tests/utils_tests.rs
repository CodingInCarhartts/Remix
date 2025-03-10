use cargo_mix::utils;

#[test]
fn test_format_size_bytes() {
    assert_eq!(utils::format_size(0), "0 bytes");
    assert_eq!(utils::format_size(1), "1 bytes");
    assert_eq!(utils::format_size(512), "512 bytes");
    assert_eq!(utils::format_size(1023), "1023 bytes");
}

#[test]
fn test_format_size_kilobytes() {
    assert_eq!(utils::format_size(1024), "1.00 KB");
    assert_eq!(utils::format_size(1536), "1.50 KB");
    assert_eq!(utils::format_size(2048), "2.00 KB");
    assert_eq!(utils::format_size(1024 * 1024 - 1), "1024.00 KB");
}

#[test]
fn test_format_size_megabytes() {
    assert_eq!(utils::format_size(1024 * 1024), "1.00 MB");
    assert_eq!(utils::format_size(1536 * 1024), "1.50 MB");
    assert_eq!(utils::format_size(1024 * 1024 * 1024 - 1), "1024.00 MB");
}

#[test]
fn test_format_size_gigabytes() {
    assert_eq!(utils::format_size(1024 * 1024 * 1024), "1.00 GB");
    assert_eq!(utils::format_size(1536 * 1024 * 1024), "1.50 GB");
    assert_eq!(utils::format_size(2048 * 1024 * 1024), "2.00 GB");
} 
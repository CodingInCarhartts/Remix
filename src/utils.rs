/// Convert bytes to a human-readable string
pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}

/// Open a file with the default system program
pub fn open_file(path: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd").args(["/C", "start", "", path]).spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open").arg(path).spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open").arg(path).spawn()?;
    }
    
    Ok(())
} 
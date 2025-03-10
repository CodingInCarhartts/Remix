mod cli;
mod comments;
mod config;
mod formatter;
mod packer;
mod remote;
mod scanner;
mod security;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use env_logger::Env;
use log::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.len() > 1 && args[1] == "mix" {
        // When run as 'cargo mix', skip the 'mix' argument
        let filtered_args: Vec<String> = args.into_iter().enumerate()
            .filter(|(i, arg)| *i != 1 || arg != "mix")
            .map(|(_, arg)| arg)
            .collect();
        Cli::parse_from(filtered_args)
    } else {
        Cli::parse()
    };
    
    // Load configuration
    let config = if let Some(ref config_path) = cli.config {
        config::load_config(config_path)
            .context(format!("Failed to load config from {}", config_path.display()))?
    } else {
        config::find_and_load_config().unwrap_or_default()
    };
    
    // If --init flag is set, create a new configuration file and exit
    if cli.init {
        config::init_config()?;
        return Ok(());
    }
    
    // Determine the target path
    let target_path = if let Some(path) = &cli.path {
        PathBuf::from(path)
    } else {
        std::env::current_dir()?
    };
    
    info!("Starting cargo-mix on {}", target_path.display());
    
    // If processing a remote repository
    if let Some(remote_url) = &cli.remote {
        let branch = cli.remote_branch.as_ref().map_or_else(|| "main".to_string(), |s| s.clone());
        info!("Processing remote repository: {} ({})", remote_url, branch);
        
        let temp_dir = remote::clone_repository(remote_url, &branch)
            .context("Failed to clone remote repository")?;
        
        let result = packer::pack_repository(
            &temp_dir, 
            &config.merge_with_cli(&cli),
        ).await?;
        
        formatter::output_result(&result, &config.output)?;
    } else {
        // Process local repository
        info!("Processing local repository: {}", target_path.display());
        
        let result = packer::pack_repository(
            &target_path, 
            &config.merge_with_cli(&cli),
        ).await?;
        
        formatter::output_result(&result, &config.output)?;
    }
    
    info!("Repository packing completed successfully");
    Ok(())
}

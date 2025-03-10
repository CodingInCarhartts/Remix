use anyhow::{Context, Result};
use git2::{BranchType, Oid, Repository};
use log::{debug, info};
use regex::Regex;
use std::path::PathBuf;
use tempfile::TempDir;
use indicatif::{ProgressBar, ProgressStyle};

/// Clone a remote repository to a temporary directory.
/// 
/// Supports various Git URL formats:
/// - Full URLs: https://github.com/username/repo
/// - GitHub shorthand: username/repo
/// - Branch URLs: https://github.com/username/repo/tree/branch
/// - Commit URLs: https://github.com/username/repo/commit/hash
pub fn clone_repository(url: &str, branch: &str) -> Result<PathBuf> {
    info!("Cloning repository: {}", url);
    
    // Create a progress bar for the clone operation
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {prefix:.bold.dim} {msg}")
            .unwrap()
    );
    progress.set_prefix("[Clone]");
    progress.set_message(format!("Parsing URL: {}", url));
    
    // Parse the URL to extract repository information
    let (repo_url, target_ref) = parse_git_url(url, branch)?;
    
    // Create temporary directory
    progress.set_message("Creating temporary directory...");
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path().to_path_buf();
    
    debug!("Cloning to temporary directory: {}", temp_path.display());
    
    // Clone the repository
    progress.set_message(format!("Cloning repository: {}...", repo_url));
    
    // Since git2 doesn't have built-in progress reporting, we'll use the spinner
    // and update it periodically to show activity
    let clone_ticker = std::thread::spawn(move || {
        let pb = progress;
        let messages = [
            "Downloading objects...",
            "Resolving deltas...",
            "Checking out files...",
            "Indexing repository...",
        ];
        let mut idx = 0;
        loop {
            pb.set_message(format!("Cloning: {}", messages[idx % messages.len()]));
            idx += 1;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
    
    // Perform the repository cloning
    let clone_result = match Repository::clone(&repo_url, &temp_path) {
        Ok(repo) => {
            // Successfully cloned, now check out specific branch/commit if requested
            if !target_ref.is_empty() && target_ref != "main" && target_ref != "master" {
                checkout_target(&repo, &target_ref)?;
            }
            Ok(())
        },
        Err(e) => Err(anyhow::anyhow!("Failed to clone repository: {}: {}", repo_url, e)),
    };
    
    // Stop the progress spinner thread
    clone_ticker.thread().unpark();
    
    // Check if clone was successful
    match clone_result {
        Ok(_) => {
            // Important: we need to leak the TempDir so it doesn't get deleted
            // when this function returns. It will be cleaned up when the program exits.
            std::mem::forget(temp_dir);
            
            info!("Repository cloned successfully to {}", temp_path.display());
            Ok(temp_path)
        },
        Err(e) => {
            // Clone error - propagate it
            Err(e)
        }
    }
}

/// Helper function to check out a specific branch or commit
fn checkout_target(repo: &Repository, target_ref: &str) -> Result<()> {
    // Try to parse as commit hash
    if let Ok(oid) = Oid::from_str(target_ref) {
        debug!("Checking out commit: {}", target_ref);
        let commit = repo.find_commit(oid)
            .context(format!("Failed to find commit: {}", target_ref))?;
            
        // Detach HEAD to the specified commit
        repo.set_head_detached(commit.id())
            .context(format!("Failed to set HEAD to commit: {}", target_ref))?;
            
        // Reset the working directory to match the commit
        let obj = repo.find_object(commit.id(), None)
            .context(format!("Failed to find object for commit: {}", target_ref))?;
            
        repo.reset(&obj, git2::ResetType::Hard, None)
            .context(format!("Failed to reset to commit: {}", target_ref))?;
    } else {
        // Try to check out the branch
        debug!("Checking out branch: {}", target_ref);
        
        // First try to find a local branch with that name
        if let Ok(branch) = repo.find_branch(target_ref, BranchType::Local) {
            repo.set_head(branch.get().name().unwrap_or(""))
                .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
        } else {
            // Try to find a remote branch
            let remote_branch_name = format!("origin/{}", target_ref);
            if let Ok(branch) = repo.find_branch(&remote_branch_name, BranchType::Remote) {
                // Create a local branch tracking the remote branch
                let _branch_ref = branch.get().name().unwrap_or("");
                repo.branch(target_ref, &repo.find_commit(branch.get().peel_to_commit()?.id())?, false)
                    .context(format!("Failed to create local branch from remote: {}", target_ref))?;
                
                // Set HEAD to the new local branch
                repo.set_head(&format!("refs/heads/{}", target_ref))
                    .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
            } else {
                // No branch found - report error
                return Err(anyhow::anyhow!("Could not find branch {} in repository", target_ref));
            }
            
            // Reset working directory to match the branch head
            let obj = repo.revparse_single(&format!("refs/heads/{}", target_ref))
                .context(format!("Failed to find reference for branch: {}", target_ref))?;
                
            repo.reset(&obj, git2::ResetType::Hard, None)
                .context(format!("Failed to reset to branch: {}", target_ref))?;
        }
    }
    
    Ok(())
}

/// Parse a Git URL into its component parts and extract the repository URL and target reference.
fn parse_git_url(url: &str, branch: &str) -> Result<(String, String)> {
    // Handle GitHub shorthand format: username/repo
    if !url.contains("://") && url.contains('/') {
        let repo_url = format!("https://github.com/{}", url);
        return Ok((repo_url, branch.to_string()));
    }
    
    // Handle branch URLs: https://github.com/username/repo/tree/branch
    let branch_regex = Regex::new(r"(?:https?://[^/]+/[^/]+/[^/]+)/tree/([^/]+)").unwrap();
    if let Some(captures) = branch_regex.captures(url) {
        let base_url = branch_regex.replace(url, "$1").to_string();
        let branch_name = captures.get(1).unwrap().as_str();
        return Ok((base_url, branch_name.to_string()));
    }
    
    // Handle commit URLs: https://github.com/username/repo/commit/hash
    let commit_regex = Regex::new(r"(?:https?://[^/]+/[^/]+/[^/]+)/commit/([^/]+)").unwrap();
    if let Some(captures) = commit_regex.captures(url) {
        let base_url = commit_regex.replace(url, "$1").to_string();
        let commit_hash = captures.get(1).unwrap().as_str();
        return Ok((base_url, commit_hash.to_string()));
    }
    
    // Default case: regular URL
    Ok((url.to_string(), branch.to_string()))
} 
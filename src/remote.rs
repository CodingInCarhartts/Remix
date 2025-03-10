use anyhow::{Context, Result};
use git2::{BranchType, Oid, Repository};
use log::{debug, info, warn};
use regex::Regex;
use std::path::PathBuf;
use tempfile::TempDir;

/// Clone a remote repository to a temporary directory.
/// 
/// Supports various Git URL formats:
/// - Full URLs: https://github.com/username/repo
/// - GitHub shorthand: username/repo
/// - Branch URLs: https://github.com/username/repo/tree/branch
/// - Commit URLs: https://github.com/username/repo/commit/hash
pub fn clone_repository(url: &str, branch: &str) -> Result<PathBuf> {
    info!("Cloning repository: {}", url);
    
    // Parse the URL to extract repository information
    let (repo_url, target_ref) = parse_git_url(url, branch)?;
    
    // Create temporary directory
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path().to_path_buf();
    
    debug!("Cloning to temporary directory: {}", temp_path.display());
    
    // Clone the repository
    let _repo = if target_ref.is_empty() || target_ref == "main" || target_ref == "master" {
        // Simple clone of the default branch
        Repository::clone(&repo_url, &temp_path)
            .context(format!("Failed to clone repository: {}", repo_url))?
    } else {
        // Clone the repository
        let repo = Repository::clone(&repo_url, &temp_path)
            .context(format!("Failed to clone repository: {}", repo_url))?;
        
        // Try to parse as commit hash
        if let Ok(oid) = Oid::from_str(&target_ref) {
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
            
            // First, try local branches
            let branch_result = repo.find_branch(&target_ref, BranchType::Local);
            
            if branch_result.is_err() {
                // Try to find a remote branch and create a local one
                let reference_name = format!("refs/remotes/origin/{}", target_ref);
                
                if let Ok(reference) = repo.find_reference(&reference_name) {
                    repo.set_head(&reference_name)
                        .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
                        
                    // Reset the working directory to match the branch
                    let obj = reference.peel(git2::ObjectType::Commit)
                        .context(format!("Failed to peel reference to commit: {}", reference_name))?;
                        
                    repo.reset(&obj, git2::ResetType::Hard, None)
                        .context(format!("Failed to reset to branch: {}", target_ref))?;
                } else {
                    warn!("Could not find branch '{}'. Using default branch instead.", target_ref);
                }
            } else {
                // Check out the local branch
                let branch = branch_result.unwrap();
                let reference = branch.get();
                
                repo.set_head(reference.name().unwrap())
                    .context(format!("Failed to set HEAD to branch: {}", target_ref))?;
                    
                // Reset the working directory to match the branch
                let obj = reference.peel(git2::ObjectType::Commit)
                    .context(format!("Failed to peel reference to commit: {}", reference.name().unwrap()))?;
                    
                repo.reset(&obj, git2::ResetType::Hard, None)
                    .context(format!("Failed to reset to branch: {}", target_ref))?;
            }
        }
        
        repo
    };
    
    debug!("Repository cloned successfully to {}", temp_path.display());
    
    // Important: we need to leak the TempDir so it doesn't get deleted
    // when this function returns. It will be cleaned up when the program exits.
    std::mem::forget(temp_dir);
    
    Ok(temp_path)
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
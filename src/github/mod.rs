pub mod repo;

use std::io::{Error, ErrorKind, Result};
use crate::config::REPO_URL;

pub fn extract_organization_from_repo_url() -> Result<String> {
    // Extract organization from REPO_URL constant
    // REPO_URL = "https://github.com/NextNodeSolutions"
    let org_name = REPO_URL
        .split('/')
        .last()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Could not extract organization from REPO_URL"))?;
    
    Ok(org_name.to_string())
}

pub async fn create_github_repository_with_code(
    token: &str,
    repo_name: &str,
    project_path: &std::path::Path,
    description: &str,
    github_tag: Option<&str>,
) -> Result<()> {
    let github_repo = repo::GitHubRepo::new(token);
    
    // Create the repository (with topic if provided)
    let repo_url = github_repo
        .create_repository(repo_name, description, false, github_tag)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create GitHub repository: {}", e)))?;
    
    println!("Created GitHub repository: {}", repo_url);
    
    // Initialize git and push the generated code (includes pnpm install results)
    github_repo
        .initialize_git_and_push(
            project_path,
            &repo_url,
            "Project Generator",
            "generator@nextnode.dev",
        )
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to initialize and push to GitHub: {}", e)))?;
    
    println!("Successfully pushed generated code to GitHub repository!");
    
    Ok(())
} 
use git2::{Repository, Signature};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};
use serde_json::json;
use std::path::Path;
use crate::config::REPO_URL;

pub struct GitHubRepo {
    token: String,
}

impl GitHubRepo {
    pub fn new(token: &str) -> Self {
        Self { 
            token: token.to_string() 
        }
    }

    pub async fn create_repository(
        &self,
        name: &str,
        description: &str,
        private: bool,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Extract organization from REPO_URL constant
        // REPO_URL = "https://github.com/NextNodeSolutions"
        let org_name = REPO_URL
            .split('/')
            .last()
            .ok_or("Could not extract organization from REPO_URL")?;
        
        // Build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|_| "Failed to create authorization header")?,
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json")
                .map_err(|_| "Failed to create accept header")?,
        );

        // Build request body
        let body = json!({
            "name": name,
            "description": description,
            "private": private,
            "auto_init": false
        });

        // Make GitHub API call
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("https://api.github.com/orgs/{}/repos", org_name))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to GitHub API: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await
                .map_err(|e| format!("Failed to read error response: {}", e))?;
            return Err(format!("GitHub API error: {}", error).into());
        }

        let repo_data: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let repo_url = repo_data["html_url"]
            .as_str()
            .ok_or("No html_url in response")?
            .to_string();

        Ok(repo_url)
    }

    pub fn initialize_and_push(
        &self,
        local_path: &Path,
        repo_url: &str,
        author_name: &str,
        author_email: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let repo = Repository::init(local_path)?;

        // Add all files
        let mut index = repo.index()?;
        index.add_path(Path::new("."))?;
        index.write()?;

        // Create initial commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = Signature::now(author_name, author_email)?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;

        // Add remote and push
        let mut remote = repo.remote("origin", repo_url)?;
        remote.push(&["refs/heads/main:refs/heads/main"], None)?;

        Ok(())
    }
}

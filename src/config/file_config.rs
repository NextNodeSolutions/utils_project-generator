use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub name: String,
    pub project_name: String,
    pub description: String,
    pub author: String,
    pub github_token: Option<String>,
    pub create_repo: bool,
    #[serde(default)]
    pub additional_vars: std::collections::HashMap<String, String>,
}

impl FileConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        // Try YAML first, then JSON
        if let Ok(config) = serde_yaml::from_str(&content) {
            Ok(config)
        } else {
            let config: FileConfig = serde_json::from_str(&content)?;
            Ok(config)
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name is required".to_string());
        }
        if self.project_name.is_empty() {
            return Err("project_name is required".to_string());
        }
        if self.description.is_empty() {
            return Err("description is required".to_string());
        }
        if self.author.is_empty() {
            return Err("author is required".to_string());
        }
        if self.create_repo && self.github_token.is_none() {
            return Err("github_token is required when create_repo is true".to_string());
        }
        Ok(())
    }
}

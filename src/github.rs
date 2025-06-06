use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use serde_json::json;
use std::fs;

pub async fn trigger_workflow(config_path: &str, token: &str) -> Result<()> {
    // Read config file content
    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))
            .context("Failed to create authorization header")?,
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_str("application/vnd.github.v3+json")
            .context("Failed to create accept header")?,
    );

    // Build request body
    let body = json!({
        "ref": "main",
        "inputs": {
            "project_config": config_content
        }
    });

    // Make GitHub API call
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.github.com/repos/{owner}/{repo}/actions/workflows/generate-project.yml/dispatches")
        .headers(headers)
        .json(&body)
        .send()
        .await
        .context("Failed to send request to GitHub API")?;

    if !response.status().is_success() {
        let error = response.text().await?;
        anyhow::bail!("GitHub API error: {}", error);
    }

    println!("Successfully triggered GitHub workflow!");
    Ok(())
}

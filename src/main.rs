mod args;
mod cli;
mod config;
mod generate;
mod github;
mod template;
mod utils;

use anyhow::{Context, Result};
use args::Args;
use clap::Parser;
use cli::{get_template_info, prompt_for_repo_name};
use generate::{handle_config_mode, handle_interactive_mode};
use github::{create_github_repository_with_code, extract_organization_from_repo_url};
use std::path::PathBuf;
use template::TemplateManager;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Set debug mode in the global context
    utils::context::set_debug_mode(args.debug);

    // Initialize template manager and clone the repository
    let template_manager = TemplateManager::new().unwrap_or_else(|err| {
        utils::error::print_error_and_exit_with_error("Failed to initialize template manager", &err)
    });

    // Get template info and path
    let (category, template_name) = get_template_info(&args, &template_manager)?;
    let template_path = template_manager.get_template_path(&category, &template_name);

    if args.remote {
        // Remote mode: generate project locally, then create GitHub repo
        let token = args
            .token
            .or_else(|| std::env::var("GITHUB_TOKEN").ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "GitHub token is required for remote mode. Set GITHUB_TOKEN env var or use --token"
                )
            })?;

        // Get organization from REPO_URL
        let organization = extract_organization_from_repo_url()?;
        println!("Using organization: {}", organization);

        // Ask for repository name
        let repo_name = prompt_for_repo_name()
            .ok_or_else(|| anyhow::anyhow!("Repository name is required"))?;

        // Generate project locally
        let project_name = if let Some(config_path) = &args.config {
            // Config mode
            let config_content = std::fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
            
            // Parse config to get project name
            let config: serde_yaml::Value = serde_yaml::from_str(&config_content)
                .context("Failed to parse config file")?;
            
            config["project_name"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("project_name is required in config file"))?
                .to_string()
        } else {
            // Interactive mode - we need to generate the project first to get the project name
            // This is a bit tricky, let's handle it differently
            anyhow::bail!("Config file is required for remote mode. Use --config to specify a config file.")
        };

        let project_path = PathBuf::from(&project_name);

        // Generate the project
        if args.config.is_some() {
            handle_config_mode(&template_path, &project_name)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        } else {
            handle_interactive_mode(&template_path)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }

        // Get description from config or use default
        let description = if let Some(config_path) = &args.config {
            let config_content = std::fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
            
            let config: serde_yaml::Value = serde_yaml::from_str(&config_content)
                .context("Failed to parse config file")?;
            
            config["description"]
                .as_str()
                .unwrap_or("Generated project")
                .to_string()
        } else {
            "Generated project".to_string()
        };

        // Create GitHub repository with the generated code
        create_github_repository_with_code(&token, &repo_name, &project_path, &description).await?;

        return Ok(());
    }

    // Handle generation based on mode
    if args.config.is_none() {
        return handle_interactive_mode(&template_path).map_err(|e| anyhow::anyhow!("{}", e));
    }

    // Get project name from variables
    let project_name = utils::context::get_variable("project_name").unwrap_or_else(|| {
        utils::error::print_error_and_exit("project_name is required in configuration file")
    });

    handle_config_mode(&template_path, &project_name).map_err(|e| anyhow::anyhow!("{}", e))
}

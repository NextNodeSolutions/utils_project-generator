mod args;
mod cli;
mod config;
mod generate;
mod github;
mod template;
mod utils;

use std::io::{Error, ErrorKind, Result};
use args::Args;
use clap::Parser;
use cli::{get_template_info, prompt_for_repo_name};
use generate::{handle_config_mode, handle_interactive_mode};
use github::{create_github_repository_with_code, extract_organization_from_repo_url};
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

    // Handle local generation first (early return)
    if !args.remote {
        // Handle generation based on mode
        if args.config.is_none() {
            return handle_interactive_mode(&template_path).map_err(|e| Error::new(ErrorKind::Other, e.to_string()));
        }

        // Get project name from variables
        let project_name = utils::context::get_variable("project_name").unwrap_or_else(|| {
            utils::error::print_error_and_exit("project_name is required in configuration file")
        });

        return handle_config_mode(&template_path, &project_name).map_err(|e| Error::new(ErrorKind::Other, e.to_string()));
    }

    // Remote mode: generate project locally, then create GitHub repo
    let token = args
        .token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                "GitHub token is required for remote mode. Set GITHUB_TOKEN env var or use --token"
            )
        })?;

    // Config file is required for remote mode - check early
    let config_path = args.config.as_ref().ok_or_else(|| {
        Error::new(ErrorKind::InvalidInput, "Config file is required for remote mode. Use --config to specify a config file.")
    })?;

    // Read and parse config file early to get project name
    let config_content = std::fs::read_to_string(config_path)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to read config file: {}", e)))?;
    
    let config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse config file: {}", e)))?;
    
    let project_name = config["project_name"]
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "project_name is required in config file"))?
        .to_string();

    // Get organization from REPO_URL
    let organization = extract_organization_from_repo_url()?;
    println!("Using organization: {}", organization);

    // Ask for repository name with option to use project name
    let repo_name = prompt_for_repo_name(&project_name)
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Repository name is required"))?;

    // Create temporary directory for remote mode
    let temp_dir = std::env::temp_dir().join(format!("project-generator-{}", project_name));
    let project_path = temp_dir;

    // Generate the project in temp directory
    if args.config.is_some() {
        crate::generate::handle_config_mode_with_path_no_deps(&template_path, &project_name, &project_path)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    } else {
        handle_interactive_mode(&template_path)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    }

    // Install dependencies AFTER copying template files but BEFORE Git operations
    crate::generate::project_generator::install_dependencies(&project_path)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to install dependencies: {}", e)))?;

    // Get description from config or use default
    let description = config["description"]
        .as_str()
        .unwrap_or("Generated project")
        .to_string();

    // Create GitHub repository and push the code (includes full Git workflow)
    create_github_repository_with_code(&token, &repo_name, &project_path, &description).await?;

    Ok(())
}

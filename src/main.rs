mod args;
mod cli;
mod config;
mod generate;
mod github;
mod template;
mod utils;

use anyhow::Result;
use args::Args;
use clap::Parser;
use cli::get_template_info;
use generate::{handle_config_mode, handle_interactive_mode};
use template::TemplateManager;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.remote {
        return github::trigger_workflow(
            args.config
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Config file is required for remote workflow"))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?,
            &args
                .token
                .or_else(|| std::env::var("GITHUB_TOKEN").ok())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "GitHub token is required. Set GITHUB_TOKEN env var or use --token"
                    )
                })?,
        )
        .await;
    }

    // Set debug mode in the global context
    utils::context::set_debug_mode(args.debug);

    // Initialize template manager and clone the repository
    let template_manager = TemplateManager::new().unwrap_or_else(|err| {
        utils::error::print_error_and_exit_with_error("Failed to initialize template manager", &err)
    });

    // Get template info and path
    let (category, template_name) = get_template_info(&args, &template_manager)?;
    let template_path = template_manager.get_template_path(&category, &template_name);

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

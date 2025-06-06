use clap::Parser;
use std::path::PathBuf;

mod cli;
mod config;
mod generate;
mod github;
mod template;
mod utils;

use config::file_config;
use generate::project_generator;
use template::TemplateManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// Path to the configuration file (YAML or JSON)
    #[arg(short = 'f', long)]
    config: Option<PathBuf>,

    /// Template category
    #[arg(short = 'c', long)]
    category: Option<String>,

    /// Template name
    #[arg(short = 'n', long)]
    template: Option<String>,

    /// Trigger GitHub workflow instead of local generation
    #[arg(long)]
    remote: bool,

    /// GitHub token for remote workflow
    #[arg(long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.remote {
        let token = args
            .token
            .or_else(|| std::env::var("GITHUB_TOKEN").ok())
            .ok_or("GitHub token is required. Set GITHUB_TOKEN env var or use --token")?;

        let config_path = args
            .config
            .ok_or("Config file is required for remote workflow")?;
        github::trigger_workflow(config_path.to_str().ok_or("Invalid config path")?, &token)
            .await?;
        return Ok(());
    }

    // Set debug mode in the global context
    utils::context::set_debug_mode(args.debug);

    // Initialize template manager and clone the repository
    let template_manager = TemplateManager::new().unwrap_or_else(|err| {
        utils::error::print_error_and_exit_with_error("Failed to initialize template manager", &err)
    });

    // Get template path
    let (category, template_name) = if let Some(config_path) = &args.config {
        // Try to get template info from config file
        match file_config::from_file(config_path) {
            Ok(config) => {
                // Set variables from config
                utils::context::set_variables(config.to_variables());

                // Get template info from config
                config.get_template_info().unwrap_or_else(|| {
                    utils::error::print_error_and_exit(
                        "template_category and template_name are required in configuration file",
                    )
                })
            }
            Err(e) => {
                eprintln!("Error reading configuration file: {}", e);
                std::process::exit(1);
            }
        }
    } else if let (Some(cat), Some(tmpl)) = (args.category, args.template) {
        (cat, tmpl)
    } else {
        // List available templates
        let templates = template_manager.list_templates().unwrap_or_else(|err| {
            utils::error::print_error_and_exit_with_error("Failed to list templates", &err)
        });

        // Select template
        cli::functions::select_template(templates)
            .unwrap_or_else(|| utils::error::print_error_and_exit("Failed to select template"))
    };

    let template_path = template_manager.get_template_path(&category, &template_name);

    // If no config file was provided, use interactive mode
    if args.config.is_none() {
        match cli::interact() {
            Ok(_) => println!("Project generated successfully"),
            Err(e) => eprintln!("Error generating project: {}", e),
        }
        return Ok(());
    }

    // Get project name from variables
    let project_name = utils::context::get_variable("project_name").unwrap_or_else(|| {
        utils::error::print_error_and_exit("project_name is required in configuration file")
    });

    let project_path = std::path::Path::new(&config::PACKAGE_ROOT_PATH)
        .join(config::CREATION_PATH)
        .join(&project_name);

    println!(
        "Generating project '{}' with template '{}' from category '{}'",
        project_name, template_name, category
    );

    project_generator::generate_project(&template_path, &project_path).unwrap_or_else(|err| {
        utils::error::print_error_and_exit_with_error(
            "An error occurred while generating the project",
            &err,
        )
    });

    match project_generator::install_dependencies(&project_path) {
        Ok(_) => println!("Project generated successfully"),
        Err(err) => utils::error::print_error_and_exit_with_error(
            "An error occurred while installing dependencies",
            &err,
        ),
    }

    Ok(())
}

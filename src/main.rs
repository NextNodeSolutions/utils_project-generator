use clap::Parser;
use project_generator_cli::config::file_config;
use project_generator_cli::generate::project_generator;
use project_generator_cli::template::TemplateManager;
use std::path::PathBuf;

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
}

fn main() {
    let args = Args::parse();

    // Set debug mode in the global context
    project_generator_cli::utils::context::set_debug_mode(args.debug);

    // Initialize template manager and clone the repository
    let template_manager = TemplateManager::new().unwrap_or_else(|err| {
        project_generator_cli::utils::error::print_error_and_exit_with_error(
            "Failed to initialize template manager",
            &err,
        )
    });

    // Get template path
    let (category, template_name) = if let (Some(cat), Some(tmpl)) = (args.category, args.template)
    {
        (cat, tmpl)
    } else {
        // List available templates
        let templates = template_manager.list_templates().unwrap_or_else(|err| {
            project_generator_cli::utils::error::print_error_and_exit_with_error(
                "Failed to list templates",
                &err,
            )
        });

        // Select template
        project_generator_cli::cli::functions::select_template(templates).unwrap_or_else(|| {
            project_generator_cli::utils::error::print_error_and_exit("Failed to select template")
        })
    };

    let template_path = template_manager.get_template_path(&category, &template_name);

    // Read configuration file if provided
    if let Some(config_path) = args.config {
        match file_config::from_file(config_path) {
            Ok(variables) => {
                project_generator_cli::utils::context::set_variables(variables);
            }
            Err(e) => {
                eprintln!("Error reading configuration file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // If no config file, use interactive mode
        match project_generator_cli::cli::interact() {
            Ok(_) => println!("Project generated successfully"),
            Err(e) => eprintln!("Error generating project: {}", e),
        }
        return;
    }

    // Get project name from variables
    let project_name = project_generator_cli::utils::context::get_variable("project_name")
        .unwrap_or_else(|| {
            project_generator_cli::utils::error::print_error_and_exit(
                "project_name is required in configuration",
            )
        });

    let project_path = std::path::Path::new(&project_generator_cli::config::PACKAGE_ROOT_PATH)
        .join(project_generator_cli::config::CREATION_PATH)
        .join(&project_name);

    println!(
        "Generating project '{}' with template '{}' from category '{}'",
        project_name, template_name, category
    );

    project_generator::generate_project(&template_path, &project_path).unwrap_or_else(|err| {
        project_generator_cli::utils::error::print_error_and_exit_with_error(
            "An error occurred while generating the project",
            &err,
        )
    });

    match project_generator::install_dependencies(&project_path) {
        Ok(_) => println!("Project generated successfully"),
        Err(err) => project_generator_cli::utils::error::print_error_and_exit_with_error(
            "An error occurred while installing dependencies",
            &err,
        ),
    }
}

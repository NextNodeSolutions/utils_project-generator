use std::collections::HashMap;
use std::path::Path;

pub mod functions;

use crate::config::{CREATION_PATH, PACKAGE_ROOT_PATH};
use crate::generate::project_generator;
use crate::template::TemplateManager;
use crate::utils::{context, error, strings};

pub fn interact() -> std::io::Result<()> {
    // Initialize template manager and clone the repository
    let template_manager = TemplateManager::new().unwrap_or_else(|err| {
        error::print_error_and_exit_with_error("Failed to initialize template manager", &err)
    });

    // List available templates
    let templates = template_manager.list_templates().unwrap_or_else(|err| {
        error::print_error_and_exit_with_error("Failed to list templates", &err)
    });

    // Select template
    let (category, template_name) = functions::select_template(templates)
        .unwrap_or_else(|| error::print_error_and_exit("Failed to select template"));

    let template_path = template_manager.get_template_path(&category, &template_name);

    // Get project name first
    let project_name = functions::prompt_for_variable(&"project_name").unwrap_or_else(|| {
        error::print_error_and_exit("An error occurred while entering project name")
    });

    // Get package name
    let package_name = functions::prompt_for_variable(&"name").unwrap_or_else(|| {
        error::print_error_and_exit("An error occurred while entering package name")
    });

    // Initialize variables with both names
    let mut variables = HashMap::from([
        ("project_name".to_string(), project_name.to_string()),
        ("name".to_string(), package_name.to_string()),
    ]);

    // Try to get additional variables from template config
    match strings::extract_unique_keys(&template_path) {
        Ok(unique_keys) => {
            for key in &unique_keys {
                if key != "project_name" && key != "name" {
                    let value = functions::prompt_for_variable(&key).unwrap_or_else(|| {
                        error::print_error_and_exit(&format!(
                            "An error occurred while entering {}",
                            &key
                        ))
                    });
                    variables.insert(key.to_string(), value);
                }
            }
        }
        Err(err) => {
            context::debug_print(&format!("Note: No template configuration found: {}", err));
            println!("Note: No template configuration found. Using basic template generation.");
        }
    }

    context::set_variables(variables);

    let project_path = Path::new(&PACKAGE_ROOT_PATH)
        .join(CREATION_PATH)
        .join(&project_name);

    println!(
        "Generating project '{}' with template '{}' from category '{}'",
        project_name, template_name, category
    );

    project_generator::generate_project(&template_path, &project_path).unwrap_or_else(|err| {
        error::print_error_and_exit_with_error(
            "An error occurred while generating the project",
            &err,
        )
    });

    match project_generator::install_dependencies(&project_path) {
        Ok(_) => Ok(()),
        Err(err) => error::print_error_and_exit_with_error(
            "An error occurred while installing dependencies",
            &err,
        ),
    }
}

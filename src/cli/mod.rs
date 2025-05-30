use std::collections::HashMap;
use std::path::Path;

mod functions;

use crate::config::{APPS_PATH, PACKAGE_ROOT_PATH};
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

    let unique_keys = strings::extract_unique_keys(&template_path).unwrap_or_else(|err| {
        error::print_error_and_exit_with_error(
            "An error occurred while extracting unique keys",
            &err,
        )
    });

    let project_name = functions::prompt_for_variable(&"project_name").unwrap_or_else(|| {
        error::print_error_and_exit("An error occurred while entering project name")
    });

    let mut variables = HashMap::from([("name".to_string(), project_name.to_string())]);
    for key in &unique_keys {
        let value = functions::prompt_for_variable(&key).unwrap_or_else(|| {
            error::print_error_and_exit(&format!("An error occurred while entering {}", &key))
        });
        variables.insert(key.to_string(), value);
    }

    context::set_variables(variables);

    let project_path = Path::new(&PACKAGE_ROOT_PATH)
        .join(APPS_PATH)
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

use std::collections::HashMap;
use std::path::Path;

mod functions;

use crate::config::{APPS_PATH, PACKAGE_ROOT_PATH, TEMPLATES_PATH};
use crate::generate::project_generator;
use crate::utils::{context, error, strings};

pub fn interact() -> std::io::Result<()> {
    let template_path = Path::new(&PACKAGE_ROOT_PATH).join(TEMPLATES_PATH);

    let templates = project_generator::list_templates(&template_path).unwrap_or_else(|err| {
        error::print_error_and_exit_with_error("An error occurred while listing templates", &err)
    });
    let selected_template = functions::select_template(templates).unwrap_or_else(|| {
        error::print_error_and_exit("An error occurred while selecting a template");
    });

    let template_path = template_path.join(&selected_template);

    let unique_keys = strings::extract_unique_keys(&template_path).unwrap_or_else(|err| {
        error::print_error_and_exit_with_error(
            "An error occurred while extracting unique keys",
            &err,
        )
    });

    println!("unique_keys: {:?}", unique_keys);

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
        "Generating project '{}' with template '{}'",
        project_name, selected_template
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

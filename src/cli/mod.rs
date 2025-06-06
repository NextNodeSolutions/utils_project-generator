use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

mod functions;

use crate::args::Args;
use crate::config::file_config;
use crate::config::{CREATION_PATH, PACKAGE_ROOT_PATH};
use crate::generate::project_generator;
use crate::template::TemplateManager;
use crate::utils::{context, strings};

pub fn get_template_info(
    args: &Args,
    template_manager: &TemplateManager,
) -> Result<(String, String)> {
    if let Some(config_path) = &args.config {
        // Try to get template info from config file
        let config = file_config::from_file(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        // Set variables from config
        context::set_variables(config.to_variables());

        // Get template info from config
        config.get_template_info().ok_or_else(|| {
            anyhow::anyhow!(
                "template_category and template_name are required in configuration file"
            )
        })
    } else if let (Some(cat), Some(tmpl)) = (&args.category, &args.template) {
        Ok((cat.clone(), tmpl.clone()))
    } else {
        // List available templates
        let templates = template_manager
            .list_templates()
            .with_context(|| "Failed to list templates")?;

        // Select template
        functions::select_template(templates)
            .ok_or_else(|| anyhow::anyhow!("Failed to select template"))
    }
}

pub fn interact() -> Result<()> {
    // Initialize template manager and clone the repository
    let template_manager =
        TemplateManager::new().with_context(|| "Failed to initialize template manager")?;

    // List available templates
    let templates = template_manager
        .list_templates()
        .with_context(|| "Failed to list templates")?;

    // Select template
    let (category, template_name) = functions::select_template(templates)
        .ok_or_else(|| anyhow::anyhow!("Failed to select template"))?;

    let template_path = template_manager.get_template_path(&category, &template_name);

    // Get project name first
    let project_name = functions::prompt_for_variable("project_name")
        .ok_or_else(|| anyhow::anyhow!("An error occurred while entering project name"))?;

    // Get package name
    let package_name = functions::prompt_for_variable("name")
        .ok_or_else(|| anyhow::anyhow!("An error occurred while entering package name"))?;

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
                    let value = functions::prompt_for_variable(key).ok_or_else(|| {
                        anyhow::anyhow!("An error occurred while entering {}", key)
                    })?;
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

    project_generator::generate_project(&template_path, &project_path)
        .with_context(|| "An error occurred while generating the project")?;

    project_generator::install_dependencies(&project_path)
        .with_context(|| "An error occurred while installing dependencies")?;

    Ok(())
}

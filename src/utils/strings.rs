use indexmap::IndexSet;
use std::{fs, path::Path};

use crate::config::{TemplateJson, TEMPLATE_CONFIG_FILE};
use crate::utils::context;

pub fn get_project_name_from_path(path: &Path) -> String {
    let project_name = path.file_name().unwrap().to_str();

    match project_name {
        Some(name) => String::from(name),
        None => String::from(""),
    }
}

pub fn read_template_config(template_path: &Path) -> std::io::Result<TemplateJson> {
    let config_path = template_path.join(TEMPLATE_CONFIG_FILE);
    context::debug_print(&format!("Looking for config file at: {:?}", config_path));
    context::debug_print(&format!("Template path exists: {}", template_path.exists()));
    context::debug_print(&format!(
        "Template path is directory: {}",
        template_path.is_dir()
    ));

    if template_path.is_dir() {
        context::debug_print("Directory contents:");
        if let Ok(entries) = fs::read_dir(template_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    context::debug_print(&format!("  - {:?}", entry.path()));
                }
            }
        }
    }

    // Check if the config file exists
    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} not found at {:?}", TEMPLATE_CONFIG_FILE, config_path),
        ));
    }

    let config_content = fs::read_to_string(&config_path)?;
    let config: TemplateJson = serde_json::from_str(&config_content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse {}: {}", TEMPLATE_CONFIG_FILE, e),
        )
    })?;
    Ok(config)
}

pub fn extract_unique_keys(template_path: &Path) -> std::io::Result<Vec<String>> {
    context::debug_print(&format!(
        "Extracting keys from template path: {:?}",
        template_path
    ));
    let config = read_template_config(template_path)?;

    let mut unique_keys = IndexSet::new();
    for file in config {
        for replacement in &file.replacements {
            if replacement.name != "name" {
                unique_keys.insert(replacement.name.clone());
            }
        }
    }

    Ok(unique_keys.into_iter().collect())
}

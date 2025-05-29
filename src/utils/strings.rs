use indexmap::IndexSet;
use std::{fs, path::Path};

use crate::config::TemplateJson;

pub fn get_project_name_from_path(path: &Path) -> String {
    let project_name = path.file_name().unwrap().to_str();

    match project_name {
        Some(name) => String::from(name),
        None => String::from(""),
    }
}

pub fn read_template_config(template_path: &Path) -> std::io::Result<TemplateJson> {
    let config_path = template_path.join("template-config.json");
    let config_content = fs::read_to_string(&config_path)?;
    let config: TemplateJson = serde_json::from_str(&config_content)?;
    Ok(config)
}

pub fn extract_unique_keys(template_path: &Path) -> std::io::Result<Vec<String>> {
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

use crate::config::TemplateJson;
use std::path::Path;

pub fn read_template_config(template_path: &Path) -> std::io::Result<TemplateJson> {
    let config_path = template_path.join("template_config.json");
    let config_content = std::fs::read_to_string(config_path)?;
    let config: TemplateJson = serde_json::from_str(&config_content)?;
    Ok(config)
}

pub fn extract_unique_keys(template_path: &Path) -> std::io::Result<Vec<String>> {
    let config = read_template_config(template_path)?;
    let mut keys = std::collections::HashSet::new();

    for template_config in config {
        for replacement in template_config.replacements {
            keys.insert(replacement.name);
        }
    }

    Ok(keys.into_iter().collect())
}

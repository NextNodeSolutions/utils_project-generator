use std::io::{Error, ErrorKind, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct FileConfig {
    #[serde(default)]
    pub project_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub template_category: Option<String>,
    #[serde(default)]
    pub template_name: Option<String>,
    #[serde(flatten)]
    pub additional_vars: std::collections::HashMap<String, String>,
}

impl FileConfig {
    pub fn get_template_info(&self) -> Option<(String, String)> {
        match (&self.template_category, &self.template_name) {
            (Some(category), Some(name)) => Some((category.clone(), name.clone())),
            _ => None,
        }
    }

    pub fn to_variables(&self) -> std::collections::HashMap<String, String> {
        let mut vars = self.additional_vars.clone();

        // Add required variables in specific order
        vars.insert("project_name".to_string(), self.project_name.clone());
        vars.insert("name".to_string(), self.name.clone());

        // Sort variables according to template_config.json
        let mut sorted_vars = HashMap::new();

        // First add required variables in specific order
        if let Some(project_name) = vars.remove("project_name") {
            sorted_vars.insert("project_name".to_string(), project_name);
        }
        if let Some(name) = vars.remove("name") {
            sorted_vars.insert("name".to_string(), name);
        }

        // Then add remaining variables in alphabetical order
        let mut remaining_keys: Vec<_> = vars.keys().collect();
        remaining_keys.sort();
        for key in remaining_keys {
            if let Some(value) = vars.get(key) {
                sorted_vars.insert(key.clone(), value.clone());
            }
        }

        sorted_vars
    }
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileConfig> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to read config file: {}", e)))?;

    // Try YAML first, then JSON
    if let Ok(config) = serde_yaml::from_str(&content) {
        Ok(config)
    } else {
        let config: FileConfig = serde_json::from_str(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse config file: {}", e)))?;
        Ok(config)
    }
}

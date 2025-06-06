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
        vars.insert("project_name".to_string(), self.project_name.clone());
        vars.insert("name".to_string(), self.name.clone());
        vars
    }
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<FileConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    // Try YAML first, then JSON
    if let Ok(config) = serde_yaml::from_str(&content) {
        Ok(config)
    } else {
        let config: FileConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

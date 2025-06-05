use std::fs;
use std::path::Path;

pub type FileConfig = std::collections::HashMap<String, String>;

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

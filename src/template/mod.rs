use git2::Repository;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use crate::config::{TEMPLATE_CATEGORIES, TEMPLATE_REPO_URL};

pub struct TemplateManager {
    repo_path: PathBuf,
}

impl TemplateManager {
    pub fn new() -> std::io::Result<Self> {
        let repo_path = tempfile::Builder::new()
            .prefix("project-templates-")
            .tempdir()?
            .path()
            .to_path_buf();

        // Clone the repository
        Repository::clone(TEMPLATE_REPO_URL, &repo_path).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to clone repository: {}", e),
            )
        })?;

        Ok(Self { repo_path })
    }

    pub fn list_templates(&self) -> std::io::Result<Vec<(String, String)>> {
        let mut templates = Vec::new();

        for category in TEMPLATE_CATEGORIES {
            let category_path = self.repo_path.join(category);
            if !category_path.exists() {
                continue;
            }

            for entry in fs::read_dir(category_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        templates.push((category.to_string(), name.to_string()));
                    }
                }
            }
        }

        templates.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        Ok(templates)
    }

    pub fn get_template_path(&self, category: &str, template: &str) -> PathBuf {
        self.repo_path.join(category).join(template)
    }
}

impl Drop for TemplateManager {
    fn drop(&mut self) {
        // The temp directory will be automatically cleaned up
    }
}

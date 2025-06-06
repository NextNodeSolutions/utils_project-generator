use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use crate::config::TemplateJson;
use crate::generate::file_operations;
use crate::utils::strings;

pub fn generate_project(template_path: &Path, project_path: &Path) -> std::io::Result<()> {
    if !template_path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Template not found"));
    }

    fs::create_dir_all(&project_path)?;
    file_operations::copy_dir_all(&template_path, &project_path)?;

    println!(
        "Project '{}' copied from template '{}' successfully",
        project_path.file_name().unwrap().to_string_lossy(),
        template_path.file_name().unwrap().to_string_lossy()
    );

    let config = strings::read_template_config(template_path)?;

    apply_template_config(&project_path, &config)?;
    Ok(())
}

fn apply_template_config(project_path: &Path, config: &TemplateJson) -> std::io::Result<()> {
    for file in config {
        for file_to_replace in &file.files_to_replace {
            let file_path = project_path.join(&file_to_replace);
            if let Err(e) = file_operations::replace_in_file(&file_path, &file.replacements) {
                println!("Error updating file {}: {}", file_to_replace, e);
            }
        }
    }
    Ok(())
}

pub fn install_dependencies(project_path: &Path) -> std::io::Result<()> {
    let status = Command::new("pnpm")
        .arg("install")
        .current_dir(project_path)
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to install dependencies",
        ));
    }
    Ok(())
}

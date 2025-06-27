use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub mod file_operations;
pub mod functions;
pub mod project_generator;

pub fn handle_interactive_mode(template_path: &Path) -> Result<()> {
    match crate::cli::interact(template_path) {
        Ok(_) => {
            println!("Project generated successfully");
            Ok(())
        }
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

pub fn handle_config_mode(template_path: &Path, project_name: &str) -> Result<()> {
    let project_path = std::path::Path::new(&crate::config::PACKAGE_ROOT_PATH)
        .join(crate::config::CREATION_PATH)
        .join(project_name);

    handle_config_mode_with_path(template_path, project_name, &project_path)
}

pub fn handle_config_mode_with_path(template_path: &Path, project_name: &str, project_path: &Path) -> Result<()> {
    println!(
        "Generating project '{}' with template '{}'",
        project_name,
        template_path.display()
    );

    project_generator::generate_project(&template_path, &project_path)
        .map_err(|e| Error::new(ErrorKind::Other, format!("An error occurred while generating the project: {}", e)))?;

    project_generator::install_dependencies(&project_path)
        .map_err(|e| Error::new(ErrorKind::Other, format!("An error occurred while installing dependencies: {}", e)))?;

    println!("Project generated successfully");
    Ok(())
}

pub fn handle_config_mode_with_path_no_deps(template_path: &Path, project_name: &str, project_path: &Path) -> Result<()> {
    println!(
        "Generating project '{}' with template '{}'",
        project_name,
        template_path.display()
    );

    project_generator::generate_project(&template_path, &project_path)
        .map_err(|e| Error::new(ErrorKind::Other, format!("An error occurred while generating the project: {}", e)))?;

    println!("Project generated successfully");
    Ok(())
}

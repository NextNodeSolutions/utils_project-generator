use inquire::Text;

use crate::utils::validation;

pub fn select_template(templates: Vec<String>) -> Option<String> {
    let selection = inquire::Select::new("Select a template :", templates).prompt();

    match selection {
        Ok(selected_template) => Some(selected_template),
        Err(_) => None,
    }
}

pub fn prompt_for_variable(variable_name: &str) -> Option<String> {
    let prompt = format!("Enter value for {}:", variable_name);

    if variable_name == "project_name" {
        return Text::new(&prompt)
            .with_validator(validation::validate_project_name)
            .prompt()
            .ok();
    }

    Text::new(&prompt).prompt().ok()
}

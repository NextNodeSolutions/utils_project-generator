use inquire::validator::Validation;
use regex::Regex;

pub fn validate_project_name(input: &str) -> Result<Validation, inquire::error::CustomUserError> {
    let regex = Regex::new(r"^[a-z][a-z0-9_-]*$").unwrap();
    if regex.is_match(input) {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(
            "Project name must be lowercase, contain only letters, numbers, hyphens, or underscores, and start with a letter".into()
        ))
    }
}

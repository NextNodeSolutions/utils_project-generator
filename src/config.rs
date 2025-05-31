pub const PACKAGE_ROOT_PATH: &str = env!("CARGO_MANIFEST_DIR");
pub const CREATION_PATH: &str = "../";
pub const TEMPLATE_REPO_URL: &str =
    "https://github.com/NextNodeSolutions/utils_project-templates.git";
pub const TEMPLATE_CATEGORIES: &[&str] = &["apps", "packages", "utils"];

pub const TEMPLATE_CONFIG_FILE: &str = "template_config.json";
pub const EXCLUDED_DIRS: &[&str] = &["node_modules", ".next", ".turbo", "dist", "build", "out"];
pub const EXCLUDED_FILES: &[&str] = &[TEMPLATE_CONFIG_FILE];

#[derive(serde::Deserialize, Debug)]
pub struct Replacement {
    pub name: String,
    pub key: String,
    pub value: String,
}

#[derive(serde::Deserialize)]
pub struct TemplateConfig {
    pub files_to_replace: Vec<String>,
    pub replacements: Vec<Replacement>,
}

pub type TemplateJson = Vec<TemplateConfig>;

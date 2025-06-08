use crate::config::{Replacement, EXCLUDED_DIRS, EXCLUDED_FILES};

use indexmap::IndexMap;
use serde_json::{self, Map, Value};
use std::path::Path;
use std::{fs, io};

use super::functions;

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if EXCLUDED_DIRS.contains(&file_name_str.as_ref()) {
            println!("Skipping excluded directory: {}", file_name_str);
            continue;
        }

        if EXCLUDED_FILES.contains(&file_name_str.as_ref()) {
            println!("Skipping excluded file: {}", file_name_str);
            continue;
        }

        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn replace_in_file(file_path: &Path, replacements: &[Replacement]) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
        replace_in_json_file(file_path, &content, replacements)
    } else {
        replace_in_text_file(file_path, &content, replacements)
    }
}

fn write_json_to_file(file_path: &Path, ordered_map: IndexMap<String, Value>) -> io::Result<()> {
    let new_json: Map<String, Value> = ordered_map.into_iter().collect();
    fs::write(file_path, serde_json::to_string_pretty(&new_json)?)
}

fn replace_in_json_file(
    file_path: &Path,
    content: &str,
    replacements: &[Replacement],
) -> io::Result<()> {
    let template_json: Map<String, Value> = serde_json::from_str(content)?;
    let mut ordered_map = functions::create_ordered_map(&template_json, replacements);
    functions::update_existing_values(&mut ordered_map, replacements);
    write_json_to_file(file_path, ordered_map)
}

fn replace_in_text_file(
    file_path: &Path,
    content: &str,
    replacements: &[Replacement],
) -> io::Result<()> {
    let mut new_content = content.to_string();

    for replacement in replacements {
        if let Some(value) = crate::utils::context::get_variable(&replacement.name) {
            let json_value = functions::convert_value_to_json(&value, &replacement.type_);
            let formatted_value = serde_json::to_string(&json_value).unwrap_or_else(|_| value);

            new_content =
                new_content.replace(&format!("{{{{{}}}}}", replacement.name), &formatted_value);

            new_content = new_content.replace(&replacement.key, &formatted_value);
        }
    }

    fs::write(file_path, new_content)
}

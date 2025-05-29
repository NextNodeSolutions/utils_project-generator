use indexmap::IndexMap;
use serde_json::{Map, Value};

use crate::config::Replacement;
use crate::utils::context;

pub fn create_ordered_map(
    template_json: &Map<String, Value>,
    replacements: &[Replacement],
) -> IndexMap<String, Value> {
    let mut ordered_map = IndexMap::new();
    for (key, value) in template_json.iter() {
        ordered_map.insert(key.clone(), value.clone());
        if key == "name" {
            insert_new_keys(&mut ordered_map, template_json, replacements);
        }
    }
    ordered_map
}

pub fn insert_new_keys(
    ordered_map: &mut IndexMap<String, Value>,
    template_json: &Map<String, Value>,
    replacements: &[Replacement],
) {
    for replacement in replacements {
        if !template_json.contains_key(&replacement.key) {
            if let Some(value) = context::get_variable(&replacement.name) {
                ordered_map.insert(replacement.key.clone(), serde_json::json!(value));
            }
        }
    }
}

pub fn update_existing_values(
    ordered_map: &mut IndexMap<String, Value>,
    replacements: &[Replacement],
) {
    for replacement in replacements {
        if let Some(value) = context::get_variable(&replacement.name) {
            if let Some(existing_value) = ordered_map.get_mut(&replacement.key) {
                *existing_value = serde_json::json!(value);
            }
        }
    }
}

pub fn replace_variables(content: &str, replacements: &[Replacement]) -> String {
    let mut new_content = content.to_string();
    for replacement in replacements {
        if let Some(value) = context::get_variable(&replacement.name) {
            let template_var = format!("{{{{{}}}}}", replacement.name);
            new_content = new_content.replace(&template_var, &value);
        }
    }
    new_content
}

use indexmap::IndexMap;
use serde_json::{Map, Value};

use crate::config::Replacement;
use crate::utils::context;

pub fn convert_value_to_json(value: &str, type_: &str) -> Value {
    context::debug_print(&format!("DEBUG - Input value: '{}'", value));
    context::debug_print(&format!("DEBUG - Type: '{}'", type_));

    match type_ {
        "array" => {
            let array_values: Vec<Value> = value
                .split(',')
                .map(|s| Value::String(s.trim().to_string()))
                .collect();
            context::debug_print(&format!("DEBUG - Array values: {:?}", array_values));
            Value::Array(array_values)
        }
        _ => Value::String(value.to_string()),
    }
}

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
                let json_value = convert_value_to_json(&value, &replacement.type_);
                ordered_map.insert(replacement.key.clone(), json_value);
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
                let json_value = convert_value_to_json(&value, &replacement.type_);
                *existing_value = json_value;
            }
        }
    }
}

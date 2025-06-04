use indexmap::IndexMap;
use serde_json::{Map, Value};

use crate::config::Replacement;
use crate::utils::context;

fn convert_value_to_json(value: &str, type_: &str) -> Value {
    println!("DEBUG - Input value: '{}'", value);
    println!("DEBUG - Type: '{}'", type_);

    match type_ {
        "array" => {
            let array_values: Vec<Value> = value
                .split(',')
                .map(|s| Value::String(s.trim().to_string()))
                .collect();
            println!("DEBUG - Array values: {:?}", array_values);
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

pub fn replace_variables(content: &str, replacements: &[Replacement]) -> String {
    let mut new_content = content.to_string();
    for replacement in replacements {
        if let Some(value) = context::get_variable(&replacement.name) {
            let template_var = format!("{{{{{}}}}}", replacement.name);
            let json_value = convert_value_to_json(&value, &replacement.type_);
            let replacement_value = serde_json::to_string(&json_value).unwrap_or_else(|_| value);
            new_content = new_content.replace(&template_var, &replacement_value);
        }
    }
    new_content
}

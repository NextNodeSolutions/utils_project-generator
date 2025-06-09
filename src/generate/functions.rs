use indexmap::IndexMap;
use serde_json::Value;

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
    template_json: &IndexMap<String, Value>,
    replacements: &[Replacement],
) -> IndexMap<String, Value> {
    let mut ordered_map = IndexMap::new();

    println!("DEBUG - Original template keys:");
    for (i, key) in template_json.keys().enumerate() {
        println!("DEBUG - Template key {}: {}", i, key);
    }

    // First, insert all existing keys in their original order
    for (key, value) in template_json.iter() {
        ordered_map.insert(key.clone(), value.clone());
    }

    println!("DEBUG - After inserting template keys:");
    for (i, key) in ordered_map.keys().enumerate() {
        println!("DEBUG - Ordered key {}: {}", i, key);
    }

    // Then, insert new keys after the "name" key if it exists
    if let Some(name_pos) = ordered_map.get_index_of("name") {
        println!("DEBUG - Found 'name' at position {}", name_pos);

        // Insert new keys
        for replacement in replacements {
            if !template_json.contains_key(&replacement.key) {
                if let Some(value) = context::get_variable(&replacement.name) {
                    let json_value = convert_value_to_json(&value, &replacement.type_);
                    ordered_map.insert(replacement.key.clone(), json_value);
                    println!("DEBUG - Adding new key: {}", replacement.key);
                }
            }
        }
    }

    ordered_map
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

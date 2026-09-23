use core::mem::replace;
use core::mem::take;

use serde_json::Map;
use serde_json::Value;

use crate::RadioInputs;

impl RadioInputs {
    /// Normalize the input values by turning their path into sub json objects
    pub fn normalize_input_paths(&mut self) {
        let old_values = take(&mut self.values);

        for (name, data) in old_values.into_iter() {
            add_by_path_in_map(&name, data, &mut self.values);
        }
    }
}

fn add_by_path(path: &str, value: Value, json: &mut Value) -> Option<Value> {
    match json {
        Value::Object(inner) => add_by_path_in_map(path, value, inner),
        _ => {
            let mut new_map = Map::new();
            add_by_path_in_map(path, value, &mut new_map);

            Some(replace(json, Value::Object(new_map)))
        }
    }
}

fn add_by_path_in_map(path: &str, value: Value, json: &mut Map<String, Value>) -> Option<Value> {
    let split = path.split_once('.');

    match split {
        Some((current_node, remaining_path)) => {
            // Not the final path. We recurse
            let current_node = json.entry(current_node).or_insert_with(|| Value::Null);

            add_by_path(remaining_path, value, current_node)
        }
        None => {
            // Found the last node! We set the value
            json.insert(path.to_string(), value)
        }
    }
}

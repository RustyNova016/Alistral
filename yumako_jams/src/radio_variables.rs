use std::collections::HashMap;

use serde_json::Value;

use crate::json::radio_input::RadioInput;

/// Represent all the variable of a radio
pub struct RadioVariables {
    values: HashMap<String, Value>,
}

impl RadioVariables {
    pub fn new_with_aliases(
        data: HashMap<String, Value>,
        aliases: HashMap<String, RadioInput>,
    ) -> Self {
        let mut inner = HashMap::new();

        for (alias, input) in aliases.clone() {
            match data.get(&alias).or(input.default.as_ref()) {
                None => {}
                Some(val) => {
                    for target in input.targets {
                        inner.insert(target, val.clone());
                    }
                }
            }
        }

        for (target, value) in data {
            // Ignore the aliased values
            if aliases.contains_key(&target) {
                continue;
            }

            inner.insert(target, value);
        }

        Self { values: inner }
    }

    /// This returns the variables of a layer
    pub fn get_layer_variables(&self, layer_name: &str) -> HashMap<String, Value> {
        let mut out = HashMap::new();

        // Look at all the variables provided
        for (key, value) in &self.values {
            let (domain, var_name) = key.split_once(".").expect("msg"); //TODO: Error

            // Is this variable for this layer?
            if domain == layer_name {
                // Is this variable an object?
                if var_name.contains(".") {
                    add_object_to_layer_vars(&mut out, value, var_name);
                } else {
                    out.insert(var_name.to_string(), value.clone());
                }
            }
        }

        out
    }
}

fn add_object_to_layer_vars(
    layer_var_data: &mut HashMap<String, Value>,
    value: &Value,
    var_name: &str,
) {
    let (layer_var, obj_var) = var_name.split_once(".").unwrap(); //TODO: ERROR

    match layer_var_data.get_mut(layer_var) {
        Some(obj) => match obj {
            Value::Object(obj) => {
                obj.insert(obj_var.to_string(), value.clone());
            }
            _ => panic!("Not an object???"), //TODO: Error
        },
        None => {
            let mut object = HashMap::new();
            object.insert(obj_var.to_string(), value.clone());
            layer_var_data.insert(layer_var.to_owned(), serde_json::to_value(object).unwrap()); //TODO: ERROR
        }
    }
}

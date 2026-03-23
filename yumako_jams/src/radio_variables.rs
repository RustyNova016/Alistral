use std::collections::HashMap;

use chrono::Duration;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tuillez::extensions::chrono_exts::DurationExt;

use crate::json::radio_input::RadioInput;

/// Represent all the variable of a radio
#[derive(Clone, Debug)]
pub struct RadioVariables {
    values: HashMap<String, Value>,
}

impl RadioVariables {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Self { values }
    }

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
    pub fn get_layer_variables(
        &self,
        layer_name: &str,
    ) -> Result<HashMap<String, Value>, crate::Error> {
        let mut out = HashMap::new();

        // Look at all the variables provided
        for (key, value) in &self.values {
            let (domain, var_name) = key
                .split_once(".")
                .ok_or_else(|| crate::Error::VariablePathError(key.to_string()))?;

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

        Ok(out)
    }

    pub fn into_hashmap(self) -> HashMap<String, Value> {
        self.values
    }

    pub fn get_as<T: DeserializeOwned>(
        &self,
        key: &str,
        type_name: &str,
    ) -> Option<Result<T, crate::Error>> {
        let data = self.values.get(key)?;

        match serde_json::from_value(data.clone()) {
            Ok(val) => Some(Ok(val)),
            Err(err) => Some(Err(crate::Error::new_variable_type_error(
                key.to_string(),
                type_name.to_string(),
                data.to_string(),
                err,
            ))),
        }
    }

    pub fn get_as_u64(&self, key: &str) -> Option<Result<u64, crate::Error>> {
        self.get_as(key, "integer")
    }

    pub fn get_as_string(&self, key: &str) -> Option<Result<String, crate::Error>> {
        self.get_as(key, "string")
    }

    pub fn get_count(&self) -> Option<Result<u64, crate::Error>> {
        self.get_as_u64("count")
    }

    pub fn get_duration(&self) -> Option<Result<Duration, crate::Error>> {
        self.get_as_string("duration").map(|res| {
            res.and_then(|dur| {
                Duration::from_human_string(&dur).map_err(|err| {
                    crate::Error::new_variable_type_error(
                        "duration".to_string(),
                        "duration_string".to_string(),
                        dur,
                        err,
                    )
                })
            })
        })
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

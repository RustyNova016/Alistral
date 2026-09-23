use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

use crate::models::radio_file::radio_input::RadioVariable;
use crate::models::radio_variables::error::LayerVariableNotObjectSnafu;
use crate::models::radio_variables::error::RadioInputsError;

pub mod error;
pub mod normalize;

/// Represent all the variable of a radio
#[derive(Clone, Debug, Default)]
pub struct RadioInputs {
    pub values: Map<String, Value>,
}

impl RadioInputs {
    pub fn new(values: Map<String, Value>) -> Self {
        Self { values }
    }

    /// Map the variable names to their target layers
    pub fn map_input_variables(&mut self, variables: HashMap<String, RadioVariable>) {
        for (alias, variable) in variables {
            let Some(variable_value) = self.values.get(&alias).cloned().or(variable.default) else {
                continue;
            };

            for target in variable.targets {
                self.values.insert(target, variable_value.clone());
            }
        }
    }

    /// This returns the variables of a layer
    pub fn get_layer_variables(
        &self,
        layer_id: &str,
    ) -> Result<Map<String, Value>, RadioInputsError> {
        let layer_input = self.values.get(layer_id).unwrap_or(&Value::Null);

        match layer_input {
            Value::Object(obj) => Ok(obj.to_owned()),
            Value::Null => Ok(Map::new()),
            _ => LayerVariableNotObjectSnafu { layer_id }.fail(),
        }
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
}

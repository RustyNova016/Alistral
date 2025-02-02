use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::modules::filters::booleans::and_filter;
use crate::modules::seeders::listen_seeder::listen_seeder;

#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    id: String,
    step_type: String,
    variables: Map<String, Value>,
}

impl Layer {
    pub fn create_step<'a>(
        self,
        stream: RadioStream<'a>,
        radio_inputs: &'a HashMap<String, Value>,
    ) -> LayerResult<'a> {
        match self.step_type.as_str() {
            "listen_seeder" => listen_seeder(stream, self.variables),
            "and_filter" => and_filter(stream, self.variables, radio_inputs),
            _ => {
                panic!("Wrong type")
            }
        }
    }

    pub fn set_variables(
        &mut self,
        mappings: &HashMap<String, String>,
        values: &HashMap<String, Value>,
    ) {
        for (input_name, input_target) in mappings {
            self.set_input_value(input_name, input_target, values);
        }

        for (input_target, value) in values {
            self.set_overwrite_value(input_target, value.clone());
        }
    }

    /// Set a variable from the input values of the user
    ///
    /// Those variables are the ones declared in the "input" field of the radio
    pub fn set_input_value(
        &mut self,
        input_name: &str,
        input_target: &str,
        values: &HashMap<String, Value>,
    ) {
        // We need to find what layer variable this is targeting
        let Some(target) = self.resolve_variable(input_target) else {
            return;
        };

        // Now, let's get the value of the variable
        let Some(value) = values.get(input_name) else {
            return;
        };

        self.variables.insert(target, value.clone());
    }

    /// Set a variable from the input values of the user
    ///
    /// Those variables are the ones the user overwrites by specifically calling the step's variable
    pub fn set_overwrite_value(&mut self, input_target: &str, value: Value) {
        // We need to find what layer variable this is targeting
        let Some(target) = self.resolve_variable(input_target) else {
            return;
        };

        self.variables.insert(target, value);
    }

    /// Return the name of the targeted variable for this layer. Return none if the variable target another layer
    pub fn resolve_variable(&self, variable_name: &str) -> Option<String> {
        let (step_id, variable_name) = variable_name.split_once(".")?;

        if step_id == self.id {
            Some(variable_name.to_owned())
        } else {
            None
        }
    }
}

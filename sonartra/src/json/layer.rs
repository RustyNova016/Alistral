use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::modules::filters::booleans::AndFilter;
use crate::modules::filters::minimum_listens::MinimumListenFilter;
use crate::modules::radio_module::RadioModule;
use crate::radio_variables::RadioVariables;

/// A layer represent a step in the radio processing. It calls a module based on the step type
#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    id: String,
    step_type: String,

    /// The default variables for the layer
    #[serde(default)]
    inputs: HashMap<String, Value>,
}

impl Layer {
    pub fn create_step<'a>(
        self,
        stream: RadioStream<'a>,
        radio_variables: &RadioVariables,
    ) -> LayerResult<'a> {
        let variables = radio_variables.get_layer_variables(&self.id);

        match self.step_type.as_str() {
            //"listen_seeder" => listen_seeder(stream, self.variables),
            "and_filter" => AndFilter::create(self.inputs, variables)?.create_stream(stream),
            "minimum_listen_filter" => {
                MinimumListenFilter::create(self.inputs, variables)?.create_stream(stream)
            }
            _ => {
                panic!("Wrong type") // TODO: Proper error
            }
        }
    }
}

use std::collections::HashMap;

use futures::stream;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::json::layer::Layer;
use crate::radio_variables::RadioVariables;

#[derive(Serialize, Deserialize, Clone)]
pub struct Radio {
    name: String,
    stack: Vec<Layer>,
    inputs: HashMap<String, String>,
}

impl Radio {
    pub fn to_stream<'a>(self, inputs: HashMap<String, Value>) -> LayerResult<'a> {
        let variables = RadioVariables::new_with_aliases(inputs, self.inputs);
        let mut stream: RadioStream = stream::empty().boxed();

        for layer in self.stack {
            stream = layer.create_step(stream, &variables)?;
        }

        Ok(stream)
    }
}

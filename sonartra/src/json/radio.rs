use std::collections::HashMap;

use futures::stream;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::json::layer::Layer;

#[derive(Serialize, Deserialize, Clone)]
pub struct Radio {
    name: String,
    stack: Vec<Layer>,
    inputs: HashMap<String, String>,
}

impl Radio {
    pub fn to_stream(self, inputs: &HashMap<String, Value>) -> LayerResult<'_> {
        let mut stream: RadioStream = stream::empty().boxed();

        for mut layer in self.stack {
            layer.set_variables(&self.inputs, inputs);
            stream = layer.create_step(stream, inputs)?;
        }

        Ok(stream)
    }
}

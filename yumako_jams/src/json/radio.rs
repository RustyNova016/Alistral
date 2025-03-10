use std::collections::HashMap;

use futures::StreamExt as _;
use futures::stream;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::json::layer::Layer;
use crate::json::radio_input::RadioInput;
use crate::modules::radio_module::LayerResult;
use crate::radio_variables::RadioVariables;

#[derive(Serialize, Deserialize, Clone)]
pub struct Radio {
    name: String,
    stack: Vec<Layer>,
    inputs: HashMap<String, RadioInput>,
}

impl Radio {
    pub fn to_stream(
        self,
        client: &YumakoClient,
        inputs: HashMap<String, Value>,
    ) -> LayerResult<'_> {
        let variables = RadioVariables::new_with_aliases(inputs, self.inputs);
        let mut stream: RadioStream = stream::empty().boxed();

        for layer in self.stack {
            stream = layer.create_step(client, stream, &variables)?;
        }

        Ok(stream)
    }
}

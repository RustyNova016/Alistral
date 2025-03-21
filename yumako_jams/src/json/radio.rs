use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    pub name: String,

    #[serde(default = "default_description")]
    pub description: String,
    
    stack: Vec<Layer>,
    inputs: HashMap<String, RadioInput>,
}

impl Radio {
    pub fn to_stream(
        self,
        client: &YumakoClient,
        inputs: RadioVariables,
    ) -> LayerResult<'_> {
        let variables = RadioVariables::new_with_aliases(inputs.into_hashmap(), self.inputs);
        let mut stream: RadioStream = stream::empty().boxed();

        for layer in self.stack {
            stream = layer.create_step(client, stream, &variables)?;
        }

        Ok(stream)
    }

    pub fn from_file<P>(path: P) -> Result<Self, crate::Error>
    where
        P: AsRef<Path>,
    {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        serde_json::from_reader(reader).map_err(crate::Error::RadioReadError)
    }
}

fn default_description() -> String {
    String::new()
}

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;
use snafu::IntoError;
use snafu::ResultExt;

use crate::models::radio_file::layer::Layer;

pub struct RadioModule<Inputs> {
    pub id: String,
    pub inputs: Inputs,
}

impl<Inputs> RadioModule<Inputs>
where
    Inputs: DeserializeOwned,
{
    /// Create the radio module from the layer description, and the user inputs
    pub fn from_layer(
        layer: &Layer,
        user_inputs: HashMap<String, Value>,
    ) -> Result<Self, RadioModuleError> {
        // Retrieve the inputs set by default on the layer
        let mut default_inputs = layer.inputs().to_owned();

        // Overwrite the inputs with the user inputs
        for (key, val) in user_inputs {
            default_inputs.insert(key, val);
        }

        // Turn the inputs into a json value...
        let input_values = serde_json::to_value(default_inputs).context(InputReadSnafu {
            layer_id: layer.id().to_owned(),
        })?;

        // ... So we can turn it into the module's inputs
        let inputs = serde_json::from_value(input_values).map_err(|err| {
            if err.to_string().starts_with("missing field") {
                // Ugly, but waiting for https://github.com/serde-rs/json/pull/865 💀
                let error = err.to_string();
                let mut parse = error.split("`");
                let _ = parse.next();

                let input = parse
                    .next()
                    .expect("If you are seeing this fail, blame `serde_json`'s error system");

                MissingInputSnafu {
                    input_path: format!("{}.{input}", layer.id()),
                }
                .build()
            } else {
                InputReadSnafu {
                    layer_id: layer.id().to_string(),
                }
                .into_error(err)
            }
        })?;

        Ok(Self {
            inputs,
            id: layer.id().to_owned(),
        })
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum RadioModuleError {
    #[snafu(display(
        "Couldn't compile the radio due to an incorrect input: {source}. \nLayer id: `{layer_id}`"
    ))]
    InputReadError {
        source: serde_json::Error,

        layer_id: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display(
        "An input was missing during the radio compilation. Please provide it \nMissing input path: `{input_path}`"
    ))]
    MissingInputError {
        input_path: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

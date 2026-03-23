use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::json::layer::Layer;

pub type LayerResult<'a> = Result<RadioStream<'a>, crate::Error>;

pub trait RadioModule: DeserializeOwned {
    fn create(layer: &Layer, user_inputs: HashMap<String, Value>) -> Result<Self, crate::Error> {
        let mut default_inputs = layer.inputs().to_owned();

        for (key, val) in user_inputs {
            default_inputs.insert(key, val);
        }

        let input_values = serde_json::to_value(default_inputs)
            .map_err(|err| crate::Error::VariableReadError(err, layer.id().to_string()))?;

        match serde_json::from_value(input_values) {
            //.map_err(|err| crate::Error::VariableReadError(err, layer.id().to_string()))
            Ok(v) => Ok(v),
            Err(err) => {
                if err.to_string().starts_with("missing field") {
                    // Ugly, but waiting for https://github.com/serde-rs/json/pull/865 💀
                    let error = err.to_string();
                    let mut parse = error.split("`");
                    let _ = parse.next();

                    Err(crate::Error::new_missing_variable_error(
                        layer.id(),
                        parse.next().expect(
                            "If you are seeing this fail, blame `serde_json`'s error system",
                        ),
                    ))
                } else {
                    Err(crate::Error::VariableReadError(err, layer.id().to_string()))
                }
            }
        }
    }

    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a>;
}

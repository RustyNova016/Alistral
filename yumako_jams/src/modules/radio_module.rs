use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::client::YumakoClient;
use crate::RadioStream;

pub type LayerResult<'a> = Result<RadioStream<'a>, crate::Error>;

pub trait RadioModule: DeserializeOwned {
    fn create(
        mut default_inputs: HashMap<String, Value>,
        user_inputs: HashMap<String, Value>,
    ) -> Result<Self, crate::Error> {
        for (key, val) in user_inputs {
            default_inputs.insert(key, val);
        }

        serde_json::from_value(
            serde_json::to_value(default_inputs).map_err(crate::Error::VariableReadError)?,
        )
        .map_err(crate::Error::VariableReadError)
    }

    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a>;
}

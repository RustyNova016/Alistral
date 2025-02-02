use core::future::ready;

use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenSeederVariables {
    user: String,
}

pub fn listen_seeder(stream: RadioStream, variables: Map<String, Value>) -> LayerResult {
    let variables: ListenSeederVariables =
        serde_json::from_value(Value::Object(variables)).unwrap();

    Ok(stream
        .filter(move |_r| {
            if variables.user != "a" {
                return ready(false);
            }

            ready(true)
        })
        .boxed())
}

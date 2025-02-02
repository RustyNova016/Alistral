use std::collections::HashMap;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use async_fn_stream::fn_stream;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::json::radio::Radio;

#[derive(Serialize, Deserialize, Clone)]
pub struct AndFilterVariables {
    other_radio: Radio,
}

pub(crate) fn and_filter<'a>(
    mut stream: RadioStream<'a>,
    variables: Map<String, Value>,
    user_inputs: &'a HashMap<String, Value>,
) -> LayerResult<'a> {
    let variables: AndFilterVariables = serde_json::from_value(Value::Object(variables)).unwrap();
    let other_radio = variables.other_radio.to_stream(user_inputs)?;

    // We create a stream here to capture the other stream compilation as part of the first poll's work
    // If we don't do that, we force having a to read a whole radio upon compilation,
    // which could be unnessecary work, as the resulting stream may never be called
    // This also allow use to keep the compilation sync
    Ok(fn_stream(move |emitter| async move {
        let other_radio_coll: Vec<RecordingWithListens> = other_radio.collect().await;

        while let Some(track) = stream.next().await {
            if other_radio_coll
                .iter()
                .any(|other_track| track.recording().mbid == other_track.recording().mbid)
            {
                emitter.emit(track).await;
            }
        }
    })
    .boxed())
}

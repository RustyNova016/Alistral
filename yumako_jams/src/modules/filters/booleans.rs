use std::collections::HashMap;

use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use streamies::Streamies;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::json::radio::Radio;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct AndFilter {
    radio: HashMap<String, Value>,
    radio_schema: Radio,
}

impl RadioModule for AndFilter {
    fn create_stream(self, mut stream: RadioStream<'_>) -> LayerResult<'_> {
        let other_radio = self.radio_schema.to_stream(self.radio)?;

        // We create a stream here to capture the other stream collection as part of the first poll's work
        // If we don't do that, we force having a to read a whole radio upon compilation,
        // which could be unnessecary work, as the resulting stream may never be called
        // This also allow us to keep the compilation sync
        Ok(try_fn_stream(async move |emitter| {
            // Collect the other radio
            let other_radio_coll = other_radio.collect_vec().await;

            // Early yield all the Errs of the other radio
            let mut other_tracks = Vec::new();
            for track in other_radio_coll {
                match track {
                    Ok(v) => other_tracks.push(v),
                    Err(err) => emitter.emit_err(err).await,
                }
            }

            // Filter the stream
            while let Some(track) = stream.next().await {
                match track {
                    Ok(track) => {
                        if other_tracks.contains(&track) {
                            emitter.emit(track).await;
                        }
                    }
                    Err(err) => {
                        emitter.emit_err(err).await;
                    }
                }
            }

            Ok(())
        })
        .boxed())
    }
}

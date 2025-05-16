use std::collections::HashMap;

use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use streamies::Streamies;
use tracing::debug;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::json::radio::Radio;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_stream::RadioStreamaExt;
use crate::radio_variables::RadioVariables;

#[derive(Serialize, Deserialize, Clone)]
pub struct AndFilter {
    radio: HashMap<String, Value>,
    radio_schema: Radio,
}

impl RadioModule for AndFilter {
    fn create_stream<'a>(
        self,
        mut stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let other_radio = self
            .radio_schema
            .to_stream(client, RadioVariables::new(self.radio))?;

        // We create a stream here to capture the other stream collection as part of the first poll's work
        // If we don't do that, we force having a to read a whole radio upon compilation,
        // which could be unnessecary work, as the resulting stream may never be called
        // This also allow us to keep the compilation sync
        Ok(try_fn_stream(async move |emitter| {
            // Collect the other radio
            let other_tracks = other_radio.to_item_stream(&emitter).collect_vec().await;

            // Filter the stream
            while let Some(track) = stream.next().await {
                match track {
                    Ok(track) => {
                        if other_tracks
                            .iter()
                            .any(|other_track| track.entity().mbid == other_track.entity().mbid)
                        {
                            emitter.emit(track).await;
                        } else {
                            debug!("Removing `{}` from the radio", track.entity().title);
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

use std::collections::HashMap;

use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;

use serde_json::Value;
use streamies::Streamies;
use tracing::debug;
use tracing::trace;

use crate::RadioStream;
use crate::YumakoClient;
use crate::json::radio::Radio;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;
use crate::radio_stream::RadioStreamaExt;
use crate::radio_variables::RadioVariables;

#[derive(Serialize, Deserialize, Clone)]
pub struct SetJoin {
    radio: HashMap<String, Value>,
    radio_schema: Radio,
    join_type: JoinOperation,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum JoinOperation {
    Inner,
    Outer,
    Left,
    Right,
    Full,
}

impl RadioModule for SetJoin {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
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
            let other_tracks = other_radio
                .to_item_stream(&emitter)
                .inspect(|elem| trace!("Collecting: {}", elem.entity().title))
                .collect_vec()
                .await;
            let mut stream = stream.to_item_stream(&emitter);
            let mut left_mbids = Vec::new();

            // Filter the stream
            while let Some(track) = stream.next().await {
                left_mbids.push(track.entity().mbid.clone());
                if check_left_track(&track, &other_tracks, self.join_type) {
                    emitter.emit(track).await;
                } else {
                    debug!("Removing `{}` from the radio", track.entity().title);
                }
            }

            for track in other_tracks {
                if check_right_track(&track, &left_mbids, self.join_type) {
                    emitter.emit(track).await;
                }
            }

            Ok(())
        })
        .boxed())
    }
}

pub fn check_left_track(item: &RadioItem, others: &[RadioItem], op: JoinOperation) -> bool {
    match op {
        JoinOperation::Full => true,
        JoinOperation::Inner => others
            .iter()
            .any(|rec| rec.entity().mbid == item.entity().mbid),
        JoinOperation::Left => others
            .iter()
            .all(|rec| rec.entity().mbid != item.entity().mbid),
        JoinOperation::Outer => others
            .iter()
            .all(|rec| rec.entity().mbid != item.entity().mbid),
        JoinOperation::Right => false,
    }
}

pub fn check_right_track(item: &RadioItem, others: &[String], op: JoinOperation) -> bool {
    match op {
        JoinOperation::Full => true,
        JoinOperation::Inner => false,
        JoinOperation::Left => false,
        JoinOperation::Outer => others.iter().all(|rec| rec != &item.entity().mbid),
        JoinOperation::Right => others.iter().all(|rec| rec != &item.entity().mbid),
    }
}

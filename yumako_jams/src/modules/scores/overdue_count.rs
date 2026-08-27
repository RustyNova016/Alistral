use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::radio_module::LayerResult;
use crate::modules::scores::ScoreMerging;
use crate::radio_stream::RadioStreamaExt as _;

#[derive(Serialize, Deserialize, Clone)]
pub struct OverdueCountScorer {
    merge: ScoreMerging,
}

impl RadioModule<OverdueCountScorer> {
    /// Add the module to the stream
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        //TODO: use current_time
        Ok(stream.map_scores(
            |t| {
                t.estimated_listen_count_for_duration(
                    Utc::now() - t.latest_listen_date().unwrap_or_else(Utc::now),
                )
            },
            self.inputs.merge,
            self.id,
        ))
    }
}

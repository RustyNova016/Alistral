use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::radio_module::LayerResult;
use crate::modules::scores::ScoreMerging;
use crate::radio_stream::RadioStreamaExt as _;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenRateScorer {
    merge: ScoreMerging,
}

impl RadioModule<ListenRateScorer> {
    /// Add the module to the stream
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream.map_scores(
            |t| {
                t.get_listen_rate(Duration::days(365))
                    .unwrap_or(Decimal::MAX)
            },
            self.inputs.merge,
            self.id,
        ))
    }
}

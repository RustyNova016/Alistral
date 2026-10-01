use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use rust_decimal::Decimal;

use crate::RadioStream;
use crate::YumakoClient;
use crate::models::radio_stream::radio_module::LayerResult;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::scores::ScoreMerging;
use crate::radio_stream::RadioStreamaExt as _;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ListenCountScorerInputs {
    #[serde(default = "default_merge")]
    merge: ScoreMerging,
}

impl RadioModule<ListenCountScorerInputs> {
    /// Add the module to the stream
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream.map_scores(
            |t| Decimal::new(i64::try_from(t.listen_count()).unwrap_or(0), 0),
            self.inputs.merge,
            self.id,
        ))
    }
}

fn default_merge() -> ScoreMerging {
    ScoreMerging::Add
}

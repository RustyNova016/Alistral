use std::collections::HashMap;

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
pub struct BumpScore {
    bumps: HashMap<String, Decimal>,
}

impl RadioModule<BumpScore> {
    /// Add the module to the stream
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream.map_scores(
            move |t| {
                *self
                    .inputs
                    .bumps
                    .get(&t.entity().mbid)
                    .unwrap_or(&Decimal::ONE)
            },
            ScoreMerging::Multiply,
            self.id,
        ))
    }
}

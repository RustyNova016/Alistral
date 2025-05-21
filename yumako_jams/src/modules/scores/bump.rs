use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::modules::scores::ScoreMerging;
use crate::radio_stream::RadioStreamaExt as _;

#[derive(Serialize, Deserialize, Clone)]
pub struct BumpScore {
    bumps: HashMap<String, Decimal>,
}

impl RadioModule for BumpScore {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        //TODO: use current_time
        Ok(stream.set_scores(
            move |t| t.score * self.bumps.get(&t.entity().mbid).unwrap_or(&Decimal::ONE),
            ScoreMerging::Replace,
        ))
    }
}

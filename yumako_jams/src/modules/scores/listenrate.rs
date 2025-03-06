use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use futures::StreamExt as _;
use futures::TryStreamExt;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::RadioModule;
use crate::modules::scores::ScoreMerging;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenRateScorer {
    merge: ScoreMerging,
}

impl RadioModule for ListenRateScorer {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .map_ok(move |mut t| {
                let score = t
                    .get_listen_rate(Duration::days(365))
                    .unwrap_or(Decimal::MAX);
                t.set_score(score, self.merge);
                t
            })
            .boxed())
    }
}

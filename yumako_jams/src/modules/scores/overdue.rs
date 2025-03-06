use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::RadioModule;
use crate::modules::scores::ScoreMerging;

#[derive(Serialize, Deserialize, Clone)]
pub struct OverdueDurationScorer {
    merge: ScoreMerging,
}

impl RadioModule for OverdueDurationScorer {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .map_ok(move |mut t| {
                let score = t.overdue_by().num_seconds();

                t.set_score(score.into(), self.merge);
                t
            })
            .boxed())
    }
}

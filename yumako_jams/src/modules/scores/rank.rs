use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
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
pub struct UserTopScorer {
    user: String,
    start_score: Decimal,
    decrease: Decimal,
    merge: ScoreMerging,
}

impl RadioModule for UserTopScorer {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        let stream = try_fn_stream(|emitter| async move {
            // First we grab the user's data
            let listen_data = Re
            

            let mut collection = DoublePriorityQueue::new();

            let mut stream = stream.to_item_stream(&emitter).map(SortItem);

            let mut stream_ended = false;
            loop {
                if !stream_ended {
                    match stream.next().await {
                        Some(val) => {
                            let score = val.score();
                            collection.push(val, score);

                            if collection.len() as u64 <= self.max_count {
                                pg_inc!();
                                continue;
                            }
                        }
                        None => stream_ended = true,
                    }
                }

                let yielded = match self.direction {
                    SortDirection::Asc => collection.pop_min(),
                    SortDirection::Desc => collection.pop_max(),
                };

                match yielded {
                    Some(val) => emitter.emit(val.0.0).await,
                    None => break,
                }
            }

            Ok(())
        });

        Ok(stream.boxed())
    }
}

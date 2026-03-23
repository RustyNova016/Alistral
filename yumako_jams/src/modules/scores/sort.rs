use core::hash::Hash;

use async_fn_stream::try_fn_stream;
use futures::StreamExt as _;
use priority_queue::DoublePriorityQueue;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;
use crate::radio_stream::RadioStreamaExt as _;

#[derive(Serialize, Deserialize, Clone)]
pub struct SortModule {
    #[serde(default = "default_direction")]
    direction: SortDirection,

    #[serde(default = "default_max_count")]
    max_count: u64,
}

impl RadioModule for SortModule {
    #[instrument(skip(self, stream), fields(indicatif.pb_show = tracing::field::Empty))]
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        let stream = try_fn_stream(|emitter| async move {
            pg_counted!(self.max_count, "Buffering Sorter");
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

#[derive(Serialize, Deserialize, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

fn default_direction() -> SortDirection {
    SortDirection::Desc
}

fn default_max_count() -> u64 {
    5000
}

struct SortItem(pub RadioItem);

impl SortItem {
    pub fn score(&self) -> Decimal {
        self.0.score
    }
}

impl Hash for SortItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.entity().id.hash(state);
    }
}

impl PartialEq for SortItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.entity().id == other.0.entity().id
    }
}

impl Eq for SortItem {}

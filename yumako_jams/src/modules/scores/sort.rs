use core::hash::Hash;

use async_fn_stream::try_fn_stream;
use futures::StreamExt as _;
use musicbrainz_db_lite::HasMBID;
use priority_queue::DoublePriorityQueue;
use serde::Deserialize;
use serde::Serialize;
use tracing::trace;
use tuillez::pg_counted;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_item::RadioItem;
use crate::models::radio_stream::radio_module::LayerResult;
use crate::models::radio_stream::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct SortModule {
    #[serde(default = "default_direction")]
    direction: SortDirection,

    #[serde(default = "default_max_count")]
    max_count: usize,
}

impl RadioModule<SortModule> {
    pub fn into_stream<'a>(
        self,
        mut stream: RadioStream<'a>,
        _: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let stream = try_fn_stream(|emitter| async move {
            pg_counted!(self.inputs.max_count, "Buffering Sorter");
            let mut collection = DoublePriorityQueue::new();

            while let Some(item) = stream.next().await {
                let item = match item {
                    Ok(item) => item,
                    Err(err) => {
                        emitter.emit_err(err).await;
                        continue;
                    }
                };

                let score = item.score;
                trace!("[{}] Received {}", self.id, item.entity().get_mbid());
                collection.push(SortItem(item), score);

                // Are we full?
                if collection.len() < self.inputs.max_count {
                    continue;
                }

                // We pop one and send it
                let yielded = match self.inputs.direction {
                    SortDirection::Asc => collection.pop_min(),
                    SortDirection::Desc => collection.pop_max(),
                };

                match yielded {
                    Some(val) => {
                        let item = val.0.0;
                        trace!(
                            "[{}] Sent {} (Score: {})",
                            self.id,
                            item.entity().get_mbid(),
                            item.score
                        );
                        emitter.emit(item).await
                    }
                    None => break,
                }
            }

            // No more elements. We yield everything else
            loop {
                let yielded = match self.inputs.direction {
                    SortDirection::Asc => collection.pop_min(),
                    SortDirection::Desc => collection.pop_max(),
                };

                match yielded {
                    Some(val) => {
                        let item = val.0.0;
                        trace!(
                            "[{}] Sent {} (Score: {})",
                            self.id,
                            item.entity().get_mbid(),
                            item.score
                        );
                        emitter.emit(item).await
                    }
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

fn default_max_count() -> usize {
    5000
}

struct SortItem(pub RadioItem);

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

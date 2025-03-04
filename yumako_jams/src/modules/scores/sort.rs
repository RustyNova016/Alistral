use core::cmp::Reverse;

use async_fn_stream::fn_stream;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;
use streamies::Streamies as _;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct SortModule {
    direction: SortDirection,
}

impl RadioModule for SortModule {
    fn create_stream(self, stream: RadioStream<'_>) -> LayerResult<'_> {
        let stream = fn_stream(|emitter| async move {
            let mut collection = stream.collect_vec().await;
            match self.direction {
                SortDirection::Asc => collection.sort_by_key(|i| i.score),
                SortDirection::Desc => collection.sort_by_key(|i| Reverse(i.score)),
            }

            for i in collection {
                emitter.emit(i).await;
            }
        });

        Ok(stream.boxed())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

use core::cmp::Reverse;

use async_fn_stream::try_fn_stream;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;

use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::RadioStream;

#[derive(Serialize, Deserialize, Clone)]
pub struct SortModule {
    direction: SortDirection,
}

impl RadioModule for SortModule {
    fn create_stream<'a>(
        self,
        mut stream: RadioStream<'a>,
        _: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let stream = try_fn_stream(|emitter| async move {
            let mut collection = Vec::new();

            while let Some(track) = stream.next().await {
                match track {
                    Ok(val) => collection.push(val),
                    Err(err) => emitter.emit_err(err).await,
                }
            }

            match self.direction {
                SortDirection::Asc => {
                    collection.sort_by_key(|i| i.score);
                }
                SortDirection::Desc => {
                    collection.sort_by_key(|i| Reverse(i.score));
                }
            }

            for i in collection {
                emitter.emit(i).await;
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

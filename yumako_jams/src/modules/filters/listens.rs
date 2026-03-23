use core::future::ready;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use futures::StreamExt;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenFilter {
    minimum: Option<usize>,
    maximum: Option<usize>,
}

impl RadioModule for ListenFilter {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .try_filter(move |ele| {
                ready(
                    (ele.listen_count() >= self.minimum.unwrap_or(0))
                        && (ele.listen_count() <= self.maximum.unwrap_or(usize::MAX)),
                )
            })
            .boxed())
    }
}

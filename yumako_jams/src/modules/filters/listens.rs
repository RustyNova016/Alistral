use core::future::ready;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use futures::StreamExt;
use futures::TryStreamExt;
use musicbrainz_db_lite::HasMBID;
use serde::Deserialize;
use serde::Serialize;
use tracing::trace;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::radio_module::LayerResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenFilter {
    minimum: Option<usize>,
    maximum: Option<usize>,
}

impl RadioModule<ListenFilter> {
    /// Add the module to the stream
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .try_filter(move |ele| {
                let listen_count = ele.listen_count();
                let minimum_listens = self.inputs.minimum.unwrap_or(0);
                if listen_count < minimum_listens {
                    trace!(
                        "[{}] Removing {}, not enough listens ({listen_count} < {minimum_listens})",
                        self.id,
                        ele.entity().get_mbid()
                    );
                    return ready(false);
                }

                let maximum_listens = self.inputs.maximum.unwrap_or(usize::MAX);
                if listen_count > maximum_listens {
                    trace!(
                        "[{}] Removing {}, too many listens ({listen_count} < {maximum_listens})",
                        self.id,
                        ele.entity().get_mbid()
                    );
                    return ready(false);
                }

                trace!(
                    "[{}] Keeping {} ({minimum_listens} <= {listen_count} <= {maximum_listens})",
                    self.id,
                    ele.entity().get_mbid()
                );
                ready(true)
            })
            .boxed())
    }
}

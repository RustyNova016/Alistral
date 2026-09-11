use core::future::ready;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt as _;
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
pub struct TimeoutFilter {
    timeouts: HashMap<String, DateTime<Utc>>,
}

impl RadioModule<TimeoutFilter> {
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .try_filter(move |radio_item| {
                if self
                    .inputs
                    .timeouts
                    .get(&radio_item.entity().mbid)
                    .is_none_or(|t| t <= &Utc::now())
                {
                    trace!("[{}] keeping {}", self.id, radio_item.entity().get_mbid());
                    ready(true)
                } else {
                    trace!(
                        "[{}] Removing {}, in timeout",
                        self.id,
                        radio_item.entity().get_mbid()
                    );
                    ready(false)
                }
            })
            .boxed())
    }
}

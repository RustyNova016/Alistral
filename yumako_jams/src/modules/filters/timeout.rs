use core::future::ready;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeoutFilter {
    timeouts: HashMap<String, DateTime<Utc>>,
}

impl RadioModule for TimeoutFilter {
    fn create_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        Ok(stream
            .try_filter(move |r| {
                ready(
                    self.timeouts
                        .get(&r.entity().mbid)
                        .is_none_or(|t| t <= &Utc::now()),
                )
            })
            .boxed())
    }
}

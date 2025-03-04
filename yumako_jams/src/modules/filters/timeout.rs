use core::future::ready;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeoutFilter {
    timeouts: HashMap<String, DateTime<Utc>>,
}

impl RadioModule for TimeoutFilter {
    fn create_stream(self, stream: RadioStream<'_>) -> LayerResult<'_> {
        Ok(stream
            .filter(move |r| {
                ready(
                    self.timeouts
                        .get(&r.entity().mbid)
                        .is_none_or(|t| t > &Utc::now()),
                )
            })
            .boxed())
    }
}

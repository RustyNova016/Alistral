use core::future::ready;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use chrono::Utc;
use futures::StreamExt as _;
use serde::Deserialize;
use serde::Serialize;
use tuillez::extensions::chrono_exts::DurationExt as _;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct CooldownFilter {
    duration: String,
}

impl RadioModule for CooldownFilter {
    fn create_stream(self, stream: RadioStream<'_>) -> LayerResult<'_> {
        let cooldown =
            Duration::from_human_string(&self.duration).map_err(|_| crate::Error::VariableDecodeError(
                "duration".to_string(),
                "The duration couldn't be parsed. Make sure it fits the `humantime` specification"
                    .to_string(),
            ))?;

        Ok(stream
            .filter(move |r| {
                let Some(last_listen_date) = r.latest_listen_date() else {
                    return ready(true);
                };

                let after_cooldown = last_listen_date + cooldown;

                ready(after_cooldown <= Utc::now())
            })
            .boxed())
    }
}

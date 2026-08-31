use core::future::ready;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use chrono::Utc;
use futures::StreamExt as _;
use futures::TryStreamExt;
use musicbrainz_db_lite::HasMBID;
use serde::Deserialize;
use serde::Serialize;
use tracing::trace;
use tuillez::extensions::chrono_exts::DurationExt as _;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::radio_module::LayerResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct CooldownFilter {
    duration: String,
}

impl RadioModule<CooldownFilter> {
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        let cooldown = Duration::from_human_string(&self.inputs.duration).map_err(|_| {
            crate::Error::VariableDecodeError(
                "duration".to_string(),
                "The duration couldn't be parsed. Make sure it fits the `humantime` specification"
                    .to_string(),
            )
        })?;

        Ok(stream
            .try_filter(move |r| {
                let Some(last_listen_date) = r.latest_listen_date() else {
                    return ready(true);
                };

                let after_cooldown = last_listen_date + cooldown;
                let now = Utc::now();

                if after_cooldown <= now {
                    trace!(
                        "[{}] Removing {}, in cooldown ({} < {after_cooldown})",
                        self.id,
                        r.entity().get_mbid(),
                        now
                    );
                    ready(false)
                } else {
                    trace!(
                        "[{}] keeping {} ({after_cooldown} < {})",
                        self.id,
                        r.entity().get_mbid(),
                        now
                    );
                    ready(true)
                }
            })
            .boxed())
    }
}

use async_fn_stream::try_fn_stream;
use chrono::Duration;
use futures::StreamExt;
use musicbrainz_db_lite::HasMBID;
use serde::Deserialize;
use serde::Serialize;
use snafu::ResultExt;
use tracing::trace;
use tuillez::extensions::chrono_exts::DurationExt;
use tuillez::extensions::chrono_exts::TimeError;

use crate::RadioStream;
use crate::YumakoClient;
use crate::models::radio_stream::radio_module::LayerResult;
use crate::models::radio_stream::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct LengthLimiterInputs {
    #[serde(default = "default_count")]
    count: u64,
    #[serde(default = "default_duration")]
    duration: String,
    #[serde(default = "default_mode")]
    mode: LengthLimiterMode,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LengthLimiterMode {
    All,
    Any,
}

impl RadioModule<LengthLimiterInputs> {
    /// Add the module to the stream
    pub fn into_stream<'a>(
        self,
        mut stream: RadioStream<'a>,
        _: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let duration =
            Duration::from_human_string(&self.inputs.duration).context(DurationParsingSnafu {
                step_id: self.id.clone(),
            })?;

        let stream = try_fn_stream(async move |emitter| {
            let mut current_count = 0;
            let mut current_duration = Duration::default();

            while let Some(item) = stream.next().await {
                let item = match item {
                    Ok(i) => i,
                    Err(err) => {
                        emitter.emit_err(err).await;
                        continue;
                    }
                };

                trace!("[{}] Passing {}", self.id, item.entity().get_mbid());

                current_duration += item.entity().length_as_duration().unwrap_or_default();
                emitter.emit(item).await;
                current_count += 1;

                let end = match self.inputs.mode {
                    LengthLimiterMode::All => {
                        current_count >= self.inputs.count && current_duration >= duration
                    }
                    LengthLimiterMode::Any => {
                        current_count >= self.inputs.count || current_duration >= duration
                    }
                };

                if end {
                    break;
                }
            }

            trace!("[{}] Blocking all remaining items", self.id);

            Ok(())
        });

        Ok(stream.boxed())
    }
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum LengthLimiterError {
    #[snafu(display(
        "Couldn't parse the duration of the step `{step_id}`.\nMake sure it fits the humantime specification: https://docs.rs/humantime/latest/humantime/fn.parse_duration.html\n\n{source}"
    ))]
    DurationParsingError {
        source: TimeError,

        step_id: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

fn default_mode() -> LengthLimiterMode {
    LengthLimiterMode::All
}

fn default_count() -> u64 {
    50
}

fn default_duration() -> String {
    "2 hours".to_string()
}

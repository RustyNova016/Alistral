use alistral_core::models::stat_periods::StatPeriod;
use chrono::DateTime;
use chrono::Utc;
use futures::StreamExt;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenInterval {
    #[serde(default = "default_min_ts")]
    min_ts: i64,

    #[serde(default = "default_max_ts")]
    max_ts: i64,

    #[serde(default = "default_period")]
    period: StatPeriod,
}

impl RadioModule for ListenInterval {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        _client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let min = self.get_start_date();
        let max = self.get_end_date();

        debug!(
            "ListenInterval: start: {min}, end: {max}, end_ts: {}",
            max.timestamp()
        );

        Ok(stream
            .map_ok(move |mut track| {
                track.recording.retain(move |l| {
                    let listened_at = l.listened_at_as_datetime();
                    min <= listened_at && listened_at <= max
                });
                track
            })
            .boxed())
    }
}

impl ListenInterval {
    pub fn get_start_date(&self) -> DateTime<Utc> {
        let min_ts = DateTime::from_timestamp(self.min_ts, 0);
        min_ts
            .unwrap_or_else(|| self.period.get_start_date())
            .max(self.period.get_start_date())
    }

    pub fn get_end_date(&self) -> DateTime<Utc> {
        let max_ts = DateTime::from_timestamp(self.max_ts, 0);
        max_ts
            .unwrap_or_else(|| self.period.get_end_date())
            .min(self.period.get_end_date())
    }
}

fn default_period() -> StatPeriod {
    StatPeriod::Last90Days
}

fn default_min_ts() -> i64 {
    0
}

fn default_max_ts() -> i64 {
    i64::MAX
}

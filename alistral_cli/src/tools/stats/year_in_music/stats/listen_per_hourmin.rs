use std::collections::HashMap;

use chrono::Duration;
use chrono::DurationRound as _;
use chrono::Timelike as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::models::hour_min::HourMinute;
use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn listen_per_hourmin_current(&self) -> &HashMap<HourMinute, Vec<Listen>> {
        self.listen_per_hourmin_current
            .get_or_init(async {
                generate_listen_per_hourmin(self.listens_with_duration_current().await)
            })
            .await
    }

    pub async fn listen_per_hourmin_previous(&self) -> &HashMap<HourMinute, Vec<Listen>> {
        self.listen_per_hourmin_previous
            .get_or_init(async {
                generate_listen_per_hourmin(self.listens_with_duration_previous().await)
            })
            .await
    }
}

fn generate_listen_per_hourmin(listens: &[(Listen, Duration)]) -> HashMap<HourMinute, Vec<Listen>> {
    let mut out: HashMap<HourMinute, Vec<Listen>> = HashMap::new();

    for (listen, recording_duration) in listens {
        let start = listen.listened_at_as_datetime();
        let end = (start + *recording_duration)
            .duration_round_up(Duration::minutes(1))
            .unwrap();
        let mut current_time = start;

        while current_time <= end {
            let time = HourMinute {
                hours: current_time.hour() as u8,
                minutes: current_time.minute() as u8,
            };

            out.entry(time).or_default().push(listen.clone());

            current_time += Duration::minutes(1)
        }
    }

    out
}

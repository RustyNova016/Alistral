use core::fmt::Write as _;

use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;
use tuillez::OwoColorize as _;

use crate::tools::lookup::lookup_components::comp_arrow::LookupCompArrow;
use crate::tools::lookup::lookup_components::duration_string::LookupDurationString;

pub struct UserLookup {
    pub(super) user: String,

    #[expect(dead_code)]
    pub(super) all_time: ListenStatisticsData,

    pub(super) now: ListenStatisticsData,
    pub(super) before: Option<ListenStatisticsData>,
}

impl UserLookup {
    pub fn new_with_timeframe(
        user: String,
        stats: ListenStatisticsData,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        let period = end - start;
        let before_start = start - period;

        let now_stats = stats.clone_no_stats().filter_listening_date(start, end);
        let before_stats = stats
            .clone_no_stats()
            .filter_listening_date(before_start, start);

        Self {
            user,
            all_time: stats,
            now: now_stats,
            before: Some(before_stats),
        }
    }

    pub fn new(stats: ListenStatisticsData, user: String) -> Self {
        Self {
            user,
            all_time: stats.clone_no_stats(),
            now: stats,
            before: None,
        }
    }

    pub async fn print_report(&self) -> String {
        let mut report = String::new();

        // Title
        writeln!(
            &mut report,
            "{}",
            format!("\n Statistics of {} ", self.user)
                .on_green()
                .black()
                .bold()
        )
        .unwrap();

        writeln!(&mut report).unwrap();
        writeln!(&mut report, "   - {}", self.get_listen_count_field()).unwrap();
        writeln!(
            &mut report,
            "   - {}",
            self.get_listen_duration_field().await
        )
        .unwrap();

        report
    }

    pub fn get_listen_count_field(&self) -> String {
        let now_data = self.now.listens().len();

        let mut string = format!("Listen count: {}", now_data);

        if let Some(data_before) = self.before.as_ref().map(|data| data.listens().len()) {
            string = format!(
                "{string} [{} {data_before}]",
                LookupCompArrow::comp_asc(now_data, data_before),
            );
        }

        string
    }

    pub async fn get_listen_duration_field(&self) -> String {
        let now_data = self.now.recording_stats().await.unwrap();
        let time = now_data.get_time_listened();

        let mut string = format!("Total playtime: {}", LookupDurationString(time));

        if let Some(before_data) = self.before.as_ref() {
            let before_data = before_data.recording_stats().await.unwrap();
            let before_time = before_data.get_time_listened();

            string = format!(
                "{string} [{} {}]",
                LookupCompArrow::comp_asc(time, before_time),
                LookupDurationString(before_time)
            );
        }

        string
    }
}

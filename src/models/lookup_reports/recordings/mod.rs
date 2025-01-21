use core::cmp::Ordering;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::datastructures::listen_timeframe::timeframe_settings::TimeframeSettings;
use alistral_core::datastructures::listen_timeframe::ListenTimeframe;
use color_eyre::owo_colors::OwoColorize as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;

use crate::models::lookup_reports::ARROW_DOWN_GREEN;
use crate::models::lookup_reports::ARROW_DOWN_RED;
use crate::models::lookup_reports::ARROW_UP_GREEN;
use crate::models::lookup_reports::ARROW_UP_RED;
use crate::models::lookup_reports::DASH_GREY;
use crate::utils::extensions::chrono_ext::DurationExt as _;

pub struct RecordingLookupReport {
    recording: Recording,
    data: ListenTimeframe<RecordingWithListensCollection>,
    all_time: bool,
}

impl RecordingLookupReport {
    pub fn new(
        listens: RecordingWithListensCollection,
        recording: Recording,
        timeframe: TimeframeSettings,
        all_time: bool,
    ) -> Self {
        let timeframe = ListenTimeframe::new(timeframe, listens);
        Self {
            data: timeframe,
            recording,
            all_time,
        }
    }

    pub fn recording_current(&self) -> Option<&RecordingWithListens> {
        self.data.current().get_by_id(self.recording.id)
    }

    pub fn recording_previous(&self) -> Option<&RecordingWithListens> {
        self.data.previous().get_by_id(self.recording.id)
    }

    pub fn all_current(&self) -> &RecordingWithListensCollection {
        self.data.current()
    }

    pub fn all_previous(&self) -> &RecordingWithListensCollection {
        self.data.previous()
    }

    fn get_arrow<T: Ord>(&self, now: &T, then: &T, lower_better: bool) -> &str {
        match now.cmp(then) {
            Ordering::Greater if lower_better => &ARROW_UP_RED,
            Ordering::Less if lower_better => &ARROW_DOWN_GREEN,
            Ordering::Greater => &ARROW_UP_GREEN,
            Ordering::Less => &ARROW_DOWN_RED,
            Ordering::Equal => &DASH_GREY,
        }
    }

    pub async fn generate_full_report(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<String, crate::Error> {
        let mut report = String::new();

        writeln!(report, "{}", self.get_title(conn).await?).unwrap();
        writeln!(report).unwrap();
        writeln!(report, "[General]").unwrap();
        writeln!(report, "   - {}", self.get_rank_line()).unwrap();
        writeln!(report, "   - {}", self.get_listen_count_line()).unwrap();
        writeln!(report, "   - {}", self.get_listen_time_line()).unwrap();

        Ok(report)
    }

    async fn get_title(&self, conn: &mut sqlx::SqliteConnection) -> Result<String, crate::Error> {
        Ok(if self.all_time {
            format!(
                "\nStatistics of {} ",
                self.recording.format_with_credits(conn).await?
            )
        } else {
            format!(
                "\nStatistics of {} ({} - {}) {}",
                self.recording.format_with_credits(conn).await?,
                self.data.settings().timeframe().start(),
                self.data.settings().timeframe().end(),
                format!(
                    "(compared to {} - {})",
                    self.data.settings().timeframe().previous_start(),
                    self.data.settings().timeframe().start()
                )
                .bright_black()
            )
        })
    }

    fn get_rank_line(&self) -> String {
        let rank = self.all_current().get_rank(&self.recording).map(|r| r + 1); // Add + 1 to make it start from 1 instead of 0
        let rank_string = rank.map(|r| r.to_string()).unwrap_or("???".to_string());

        if self.all_time {
            format!("Rank: #{rank_string}")
        } else {
            let prev_rank = self.all_previous().get_rank(&self.recording).map(|r| r + 1);
            let prev_rank_string = prev_rank
                .map(|r| r.to_string())
                .unwrap_or("???".to_string());

            format!(
                "Rank: #{} [{} - {}]",
                rank_string,
                self.get_arrow(
                    &rank.unwrap_or(usize::MAX),
                    &prev_rank.unwrap_or(usize::MAX),
                    true
                ),
                prev_rank_string
            )
        }
    }

    fn get_listen_count_line(&self) -> String {
        let count = self
            .recording_current()
            .map(|e| e.listen_count())
            .unwrap_or(0);

        if self.all_time {
            format!("Listen counts: {count}")
        } else {
            let prev_count = self
                .recording_previous()
                .map(|e| e.listen_count())
                .unwrap_or(0);

            format!(
                "Listen counts: {} [{} - {}]",
                count,
                self.get_arrow(&count, &prev_count, false),
                prev_count
            )
        }
    }

    fn get_listen_time_line(&self) -> String {
        let data = self
            .recording_current()
            .and_then(|e| e.get_time_listened())
            .map(|dur| dur.deci_minutes().trunc_with_scale(2))
            .unwrap_or(Decimal::ZERO);

        if self.all_time {
            format!("Total playtime: {data}")
        } else {
            let prev_data = self
                .recording_previous()
                .and_then(|e| e.get_time_listened())
                .map(|dur| dur.deci_minutes().trunc_with_scale(2))
                .unwrap_or(Decimal::ZERO);

            format!(
                "Total playtime: {} [{} - {}]",
                data,
                self.get_arrow(&data, &prev_data, false),
                prev_data
            )
        }
    }
}

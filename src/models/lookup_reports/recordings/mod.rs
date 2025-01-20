pub mod listen_count_line;
pub mod rank_line;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_timeframe::timeframe_settings::TimeframeSettings;
use alistral_core::datastructures::listen_timeframe::ListenTimeframe;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::models::lookup_reports::recordings::rank_line::RankLine;

pub struct RecordingLookupReport {
    recording: Recording,
    data: ListenTimeframe<RecordingWithListensCollection>,
}

impl RecordingLookupReport {
    pub async fn new(
        listens: RecordingWithListensCollection,
        recording: Recording,
        timeframe: TimeframeSettings,
    ) -> Self {
        let timeframe = ListenTimeframe::new(timeframe, listens);
        Self {
            data: timeframe,
            recording,
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

    pub async fn generate_full_report(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<String, crate::Error> {
        let mut report = String::new();

        writeln!(
            report,
            "\n Statistics of {} ",
            self.recording.format_with_credits(conn).await?
        )
        .unwrap();
        writeln!(report).unwrap();
        writeln!(report, "[General]").unwrap();
        writeln!(report, "   - {}", RankLine::from_report(self)).unwrap();
        writeln!(report, "").unwrap();
        writeln!(report, "").unwrap();
        writeln!(report, "").unwrap();
        writeln!(report, "").unwrap();

        Ok(report)
    }
}

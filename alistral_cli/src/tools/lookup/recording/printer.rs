use core::fmt::Write as _;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;
use musicbrainz_db_lite::HasArtistCredits as _;
use musicbrainz_db_lite::Recording;
use tuillez::OwoColorize as _;

use crate::ALISTRAL_CLIENT;

pub struct RecordingLookup {
    pub(super) recording: Recording,

    pub(super) all_time: ListenStatisticsData,

    pub(super) now: ListenStatisticsData,
    pub(super) before: Option<ListenStatisticsData>,
}

impl RecordingLookup {
    pub fn new_with_timeframe(
        stats: ListenStatisticsData,
        recording: Recording,
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
            recording,
            all_time: stats,
            now: now_stats,
            before: Some(before_stats),
        }
    }

    pub fn new(stats: ListenStatisticsData, recording: Recording) -> Self {
        Self {
            recording,
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
            format!(
                "\n Statistics of {} ",
                self.recording
                    .format_with_credits(
                        &mut *ALISTRAL_CLIENT.get_conn().await,
                        &ALISTRAL_CLIENT.musicbrainz_db
                    )
                    .await
                    .unwrap()
            )
            .on_green()
            .black()
            .bold()
        )
        .unwrap();

        writeln!(&mut report).unwrap();
        writeln!(&mut report, "[General]").unwrap();
        writeln!(&mut report, "   - {}", self.get_rank_field().await).unwrap();
        writeln!(&mut report, "   - {}", self.get_listen_count_field().await).unwrap();
        writeln!(&mut report, "   - {}", self.get_playtime_field().await).unwrap();
        writeln!(
            &mut report,
            "   - {}",
            self.get_first_listen_date_field().await
        )
        .unwrap();
        writeln!(
            &mut report,
            "   - {}",
            self.get_last_listen_date_field().await
        )
        .unwrap();

        writeln!(&mut report).unwrap();
        writeln!(&mut report, "{}", self.get_listen_rate_section().await).unwrap();

        report
    }

    pub async fn now_recording_stats(&self) -> &RecordingWithListensCollection {
        self.now.recording_stats().await.unwrap()
    }
    pub async fn before_recording_stats(&self) -> Option<&RecordingWithListensCollection> {
        Some(self.before.as_ref()?.recording_stats().await.unwrap())
    }

    pub async fn get_now_target_recording_stats(&self) -> &RecordingWithListens {
        self.now
            .recording_stats()
            .await
            .unwrap()
            .get_by_mbid(&self.recording.mbid)
            .unwrap()
    }

    pub async fn get_before_target_recording_stats(&self) -> Option<&RecordingWithListens> {
        self.before
            .as_ref()?
            .recording_stats()
            .await
            .unwrap()
            .get_by_mbid(&self.recording.mbid)
    }

    pub async fn get_all_time_target_recording_stats(&self) -> &RecordingWithListens {
        self.all_time
            .recording_stats()
            .await
            .unwrap()
            .get_by_mbid(&self.recording.mbid)
            .unwrap()
    }
}

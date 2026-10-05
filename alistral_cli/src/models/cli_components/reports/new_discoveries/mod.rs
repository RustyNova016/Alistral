use core::fmt::Write as _;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;
use musicbrainz_db_lite::HasRowID;
use snafu::ResultExt;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::cli_components::reports::new_discoveries::discovered_counts::discovery_counts;
use crate::models::cli_components::reports::new_discoveries::discovered_duration::discovery_duration;
use crate::models::cli_components::reports::new_discoveries::discovered_listens::discovery_listens;
use crate::models::cli_components::reports::new_discoveries::no_discoveries_with_previous::no_discoveries_with_previous;
use crate::models::cli_components::reports::utils::display_table::create_report_top_table;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;

pub mod discovered_counts;
pub mod discovered_duration;
pub mod discovered_listens;
pub mod no_discoveries_with_previous;

pub async fn discoveries_report(
    all_stats: &ListenStatisticsData,
    current_stats: &ListenStatisticsData,
    previous_stats: &ListenStatisticsData,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    previous_start_time: DateTime<Utc>,
) -> Result<String, TopDiscoveriesReportError> {
    let mut out = String::new();
    writeln!(out, "{}", Heading1("Discovered Recordings 🔎")).unwrap();
    writeln!(out).unwrap();

    let all_time_recordings = all_stats
        .recording_stats()
        .await
        .context(RecordingStatsSnafu)?;

    let current_recordings = current_stats
        .recording_stats()
        .await
        .context(RecordingStatsSnafu)?;

    let previous_recordings = previous_stats
        .recording_stats()
        .await
        .context(RecordingStatsSnafu)?;

    let current_discoveries = get_discovered_recordings(
        all_time_recordings,
        &current_recordings,
        start_time,
        end_time,
    );
    let previous_discoveries = get_discovered_recordings(
        all_time_recordings,
        &previous_recordings,
        previous_start_time,
        start_time,
    );

    match (
        current_discoveries.is_empty(),
        previous_discoveries.is_empty(),
    ) {
        (true, true) => {
            writeln!(out, "No new tracks have been discovered").unwrap();
        }
        (true, false) => {
            writeln!(
                out,
                "{}",
                no_discoveries_with_previous(&previous_discoveries)
            )
            .unwrap();
        }
        (false, true) | (false, false) => {
            writeln!(
                out,
                "{}",
                discovery_counts(
                    &current_discoveries,
                    &previous_discoveries,
                    &current_recordings,
                    &previous_recordings
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                discovery_listens(
                    &current_discoveries,
                    &previous_discoveries,
                    &current_recordings,
                    &previous_recordings
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                discovery_duration(
                    &current_discoveries,
                    &previous_discoveries,
                    &current_recordings,
                    &previous_recordings
                )
            )
            .unwrap();

            let table: TopTable<TopListenDurCountRow<_>, OrderTableByListenDuration> = TopTable::from_entity_listens(
                current_discoveries,
                OrderTableByListenDuration,
                true,
            );
            writeln!(out, "{}", table.format(20, 0).await).unwrap();
        }
    }

    Ok(out)
}

fn get_discovered_recordings(
    all_stats: &RecordingWithListensCollection,
    timeframe_stats: &RecordingWithListensCollection,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> RecordingWithListensCollection {
    let mut discovered = RecordingWithListensCollection::new();

    for recording in all_stats.iter() {
        let Some(date_discovered) = recording.oldest_listen_date() else {
            continue;
        };

        if !(start_time <= date_discovered && date_discovered <= end_time) {
            continue;
        }

        let Some(new_recording) = timeframe_stats.get_by_id(recording.rowid()) else {
            continue;
        };

        discovered.insert_or_merge_entity_stats(new_recording.clone())
    }

    discovered
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum TopDiscoveriesReportError {
    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for TopDiscoveriesReportError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RecordingStatsError { .. } => None,
        }
    }
}

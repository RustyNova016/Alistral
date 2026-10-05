pub mod discoveries_with_previous;
use core::fmt::Write as _;

use alistral_core::cli::colors::AlistralColors as _;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;
use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use snafu::ResultExt;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::reports::new_discoveries::discoveries_no_previous::discoveries_no_previous;
use crate::models::cli_components::reports::new_discoveries::discoveries_with_previous::discoveries_with_previous;
use crate::models::cli_components::reports::new_discoveries::no_discoveries_with_previous::no_discoveries_with_previous;

pub mod discoveries_no_previous;
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

    let discovered = get_discovered_recordings(all_time_recordings, start_time, end_time);
    let previous_discovered =
        get_discovered_recordings(all_time_recordings, previous_start_time, start_time);

    match (discovered.is_empty(), previous_discovered.is_empty()) {
        (true, true) => {
            writeln!(out, "No new tracks have been discovered").unwrap();
        }
        (true, false) => {
            writeln!(
                out,
                "{}",
                no_discoveries_with_previous(&previous_discovered)
            )
            .unwrap();
        }
        (false, true) => {
            let current_recordings = current_stats
                .recording_stats()
                .await
                .context(RecordingStatsSnafu)?;

            writeln!(
                out,
                "{}",
                discoveries_no_previous(&discovered, current_recordings, start_time, end_time)
            )
            .unwrap();
        }
        (false, false) => {
            let current_recordings = current_stats
                .recording_stats()
                .await
                .context(RecordingStatsSnafu)?;

            let previous_recordings = current_stats
                .recording_stats()
                .await
                .context(RecordingStatsSnafu)?;

            writeln!(
                out,
                "{}",
                discoveries_with_previous(&discovered, &previous_discovered, current_recordings, previous_recordings, start_time, end_time)
            )
            .unwrap();
        }
    }

    Ok(out)
}

fn get_discovered_recordings(
    all_stats: &RecordingWithListensCollection,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Vec<&RecordingWithListens> {
    let mut discovered = Vec::new();

    for recording in all_stats.iter() {
        let Some(date_discovered) = recording.oldest_listen_date() else {
            continue;
        };

        if start_time <= date_discovered && date_discovered <= end_time {
            discovered.push(recording)
        }
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

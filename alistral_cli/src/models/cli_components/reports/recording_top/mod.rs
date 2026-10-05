use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use itertools::Itertools;
use snafu::ResultExt;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;
use crate::utils::extensions::sequelles::hashjoin::HashJoin;

pub async fn top_recordings_report(
    current_stats: &ListenStatisticsData,
    previous_stats: &ListenStatisticsData,
    // Settings
) -> Result<String, TopRecordingReportError> {
    let mut out = String::new();
    writeln!(out, "{}", Heading1("Top recordings 🏆")).unwrap();
    writeln!(out).unwrap();

    let current_recordings = current_stats
        .recording_stats()
        .await
        .context(RecordingStatsSnafu)?;
    let previous_recordings = previous_stats
        .recording_stats()
        .await
        .context(RecordingStatsSnafu)?;

    // === Stats lines ===

    let current = current_recordings.iter_entities().count();
    let previous = previous_recordings.iter_entities().count();

    writeln!(
        out,
        "You listened to {} distinct recordings [{} {}]",
        current.alistral_green(),
        ComparisonArrow::greater_is_better(current, previous),
        previous.alistral_green(),
    )
    .unwrap();

    if !current_recordings.is_empty() || !previous_recordings.is_empty() {
        create_top_table(&mut out, current_recordings, previous_recordings).await;
    }

    Ok(out)
}

async fn create_top_table(
    out: &mut String,
    current_recordings: &RecordingWithListensCollection,
    previous_recordings: &RecordingWithListensCollection,
) {
    writeln!(out).unwrap();
    writeln!(out, "Here's the top 20 tracks of this year:").unwrap();

    let join = current_recordings.0.hash_join(&previous_recordings.0);
    let stats = join
        .into_values()
        .map(|(current, previous)| {
            EntityListensComparison::new(current.cloned(), previous.cloned())
        })
        .collect_vec();

    let table_string = { //if self.listen_counts {
        let table: TopTable<TopListenDurCountRow<_>, OrderTableByListenDuration> =
            TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
        table.format(20, 0).await
    }; //else {
    //     let table: TopTable<TopListenDurationRow<_>, OrderTableByListenDuration> =
    //         TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
    //     table.format(20, 0).await
    // };
    writeln!(out, "{table_string}").unwrap();
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum TopRecordingReportError {
    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for TopRecordingReportError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RecordingStatsError { .. } => None,
        }
    }
}

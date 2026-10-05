use core::fmt;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;

use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::formaters::mh_duration_formater::MHDurationFormater;
use crate::models::cli_components::formaters::title::Title;

pub async fn general_stats_report(
    current_stats: &ListenStatisticsData,
    previous_stats: &ListenStatisticsData,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<String, fmt::Error> {
    let mut out = String::new();

    let current_count = current_stats.listens().listen_count();
    let old_count = previous_stats.listens().listen_count();

    let current_dur = current_stats
        .recording_stats()
        .await
        .unwrap()
        .get_time_listened();
    let old_dur = previous_stats
        .recording_stats()
        .await
        .unwrap()
        .get_time_listened();

    writeln!(
        out,
        "{}",
        Title::new(format!(
            "Listen Report ({} - {})",
            start_time.format("%d/%m/%Y"),
            end_time.format("%d/%m/%Y")
        ))
    )?;
    writeln!(out)?;
    writeln!(out, "In this period you:")?;
    writeln!(
        out,
        "  - You made {} listens [{} {}]",
        current_count,
        ComparisonArrow::greater_is_better(current_count, old_count),
        old_count,
    )?;
    writeln!(
        out,
        "  - That's {} [{} {}]",
        MHDurationFormater(current_dur),
        ComparisonArrow::greater_is_better(current_dur, old_dur),
        MHDurationFormater(old_dur),
    );
    writeln!(out)?;

    Ok(out)
}

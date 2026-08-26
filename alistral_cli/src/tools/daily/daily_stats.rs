use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Days;
use chrono::Local;

use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::formaters::mh_duration_formater::MHDurationFormater;
use crate::models::cli_components::formaters::title::Title;

pub(super) async fn daily_stats(stats: &ListenStatisticsData, today: DateTime<Local>) {
    let (current, old) = stats.comparison_split(today.to_utc(), (today + Days::new(1)).to_utc());

    let current_count = current.listens().listen_count();
    let old_count = old.listens().listen_count();

    let current_dur = current.recording_stats().await.unwrap().get_time_listened();
    let old_dur = old.recording_stats().await.unwrap().get_time_listened();

    println!(
        "{}",
        Title::new(format!("Statistics for {}", today.format("%d/%m/%Y"),))
    );
    println!();
    println!("On this day you:");
    println!(
        "  - You made {} listens [{} {}]",
        current_count,
        ComparisonArrow::greater_is_better(current_count, old_count),
        old_count,
    );
    println!(
        "  - That's {} [{} {}]",
        MHDurationFormater(current_dur),
        ComparisonArrow::greater_is_better(current_dur, old_dur),
        MHDurationFormater(old_dur),
    );
    println!();
}

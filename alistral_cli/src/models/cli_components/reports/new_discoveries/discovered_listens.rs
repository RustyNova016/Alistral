use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use rust_decimal::Decimal;

use crate::models::cli_components::comp_arrow::ComparisonArrow;

pub(super) fn discovery_listens(
    current_discoveries: &RecordingWithListensCollection,
    previous_discoveries: &RecordingWithListensCollection,

    current_recordings: &RecordingWithListensCollection,
    previous_recordings: &RecordingWithListensCollection,
) -> String {
    let current_discovered_count = current_discoveries.listen_count();
    let previous_discovered_count = previous_discoveries.listen_count();

    let current_track_count = current_recordings.listen_count();
    let previous_track_count = previous_recordings.listen_count();

    let current_percent_discovered = (Decimal::new(current_discovered_count as i64, 0)
        / Decimal::new(current_track_count as i64, 0))
    .checked_mul(Decimal::ONE_HUNDRED)
    .unwrap()
    .trunc_with_scale(2);

    let previous_percent_discovered = (Decimal::new(previous_discovered_count as i64, 0)
        / Decimal::new(previous_track_count as i64, 0))
    .checked_mul(Decimal::ONE_HUNDRED)
    .unwrap()
    .trunc_with_scale(2);

    format!(
        "Which make up a total of {} listens [{} {}] ({}% of the total listens [{} {}%] )",
        current_discovered_count.alistral_green(),
        ComparisonArrow::greater_is_better(current_discovered_count, previous_discovered_count),
        previous_discovered_count.alistral_green(),
        current_percent_discovered.alistral_green(),
        ComparisonArrow::greater_is_better(current_percent_discovered, previous_percent_discovered),
        previous_percent_discovered.alistral_green()
    )
}

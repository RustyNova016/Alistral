use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use rust_decimal::Decimal;
use tuillez::extensions::chrono_exts::DurationExt;

use crate::models::cli_components::comp_arrow::ComparisonArrow;

pub(super) fn discovery_duration(
    current_discoveries: &RecordingWithListensCollection,
    previous_discoveries: &RecordingWithListensCollection,

    current_recordings: &RecordingWithListensCollection,
    previous_recordings: &RecordingWithListensCollection,
) -> String {
    let current_discovered_count = current_discoveries.get_time_listened().unwrap_or_default();
    let previous_discovered_count = previous_discoveries.get_time_listened().unwrap_or_default();

    let current_track_count = current_recordings.get_time_listened().unwrap_or_default();
    let previous_track_count = previous_recordings.get_time_listened().unwrap_or_default();

    let current_percent_discovered = (Decimal::new(current_discovered_count.num_seconds(), 0)
        / Decimal::new(current_track_count.num_seconds(), 0))
    .checked_mul(Decimal::ONE_HUNDRED)
    .unwrap()
    .trunc_with_scale(2);

    let previous_percent_discovered = (Decimal::new(previous_discovered_count.num_seconds(), 0)
        / Decimal::new(previous_track_count.num_seconds(), 0))
    .checked_mul(Decimal::ONE_HUNDRED)
    .unwrap()
    .trunc_with_scale(2);

    format!(
        "Corresponding to {} [{} {}] ({}% of the total listened time [{} {}%])",
        current_discovered_count
            .to_humantime()
            .unwrap()
            .alistral_green(),
        ComparisonArrow::greater_is_better(current_discovered_count, previous_discovered_count),
        previous_discovered_count
            .to_humantime()
            .unwrap()
            .alistral_green(),
        current_percent_discovered.alistral_green(),
        ComparisonArrow::greater_is_better(current_percent_discovered, previous_percent_discovered),
        previous_percent_discovered.alistral_green()
    )
}

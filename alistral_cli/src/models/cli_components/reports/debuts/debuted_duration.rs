use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::models::listen_with_time::iterator::IntoListenWithDurationIterator;
use rust_decimal::Decimal;

use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::formaters::human_duration::HumanDurationFormat;

pub(super) fn discovery_duration<Ent, Lis>(
    current_discoveries: EntityWithListensCollection<Ent, Lis>,
    previous_discoveries: EntityWithListensCollection<Ent, Lis>,

    current_recordings: EntityWithListensCollection<Ent, Lis>,
    previous_recordings: EntityWithListensCollection<Ent, Lis>,
) -> String
where
    EntityWithListensCollection<Ent, Lis>: IntoListenWithDurationIterator,
{
    let current_discovered_count = current_discoveries.total_listened_duration();
    let previous_discovered_count = previous_discoveries.total_listened_duration();

    let current_track_count = current_recordings.total_listened_duration();
    let previous_track_count = previous_recordings.total_listened_duration();

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
        HumanDurationFormat::from(current_discovered_count).alistral_green(),
        ComparisonArrow::greater_is_better(current_discovered_count, previous_discovered_count),
        HumanDurationFormat::from(previous_discovered_count).alistral_green(),
        current_percent_discovered.alistral_green(),
        ComparisonArrow::greater_is_better(current_percent_discovered, previous_percent_discovered),
        previous_percent_discovered.alistral_green()
    )
}

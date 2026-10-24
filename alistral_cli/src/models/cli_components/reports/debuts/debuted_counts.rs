use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use rust_decimal::Decimal;

use crate::models::cli_components::comp_arrow::ComparisonArrow;

pub(super) fn discovery_counts<Ent, Lis>(
    current_discoveries: &EntityWithListensCollection<Ent, Lis>,
    previous_discoveries: &EntityWithListensCollection<Ent, Lis>,

    current_recordings: &EntityWithListensCollection<Ent, Lis>,
    previous_recordings: &EntityWithListensCollection<Ent, Lis>,
) -> String
where
    Ent: MusicbrainzEntity,
{
    let current_discovered_count = current_discoveries.entity_count();
    let previous_discovered_count = previous_discoveries.entity_count();

    let current_track_count = current_recordings.entity_count();
    let previous_track_count = previous_recordings.entity_count();

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
        "You listened to {} new {}s that debuted in this period [{} {}] ({}% of the total of {}s listened [{} {}%])",
        current_discovered_count.alistral_green(),
        Ent::entity_name(),
        ComparisonArrow::greater_is_better(current_discovered_count, previous_discovered_count),
        previous_discovered_count.alistral_green(),
        current_percent_discovered.alistral_green(),
        Ent::entity_name(),
        ComparisonArrow::greater_is_better(current_percent_discovered, previous_percent_discovered),
        previous_percent_discovered.alistral_green()
    )
}

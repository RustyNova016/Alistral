use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::reports::utils::display_table::create_report_top_table;

pub(super) async fn generic_top_report<Ent, Lis>(
    current_entity: &EntityWithListensCollection<Ent, Lis>,
    previous_entity: &EntityWithListensCollection<Ent, Lis>,
    // Settings
) -> String
where
    Ent: HasRowID
        + MusicbrainzEntity
        + Clone
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
    Lis: ListenCollectionReadable + Default,
    EntityWithListens<Ent, Lis>: ListenCollWithTime + Clone,
{
    let mut out = String::new();
    writeln!(
        out,
        "{}",
        Heading1(format!("Top {}s 🏆", Ent::entity_name()))
    )
    .unwrap();
    writeln!(out).unwrap();

    // === Stats lines ===

    let current = current_entity.iter_entities().count();
    let previous = previous_entity.iter_entities().count();

    writeln!(
        out,
        "You listened to {} distinct {}s [{} {}]",
        current.alistral_green(),
        Ent::entity_name(),
        ComparisonArrow::greater_is_better(current, previous),
        previous.alistral_green(),
    )
    .unwrap();

    if !current_entity.is_empty() || !previous_entity.is_empty() {
        create_report_top_table(
            &mut out,
            current_entity,
            previous_entity,
            &format!("{}s", Ent::entity_name()),
        )
        .await;
    }

    out
}

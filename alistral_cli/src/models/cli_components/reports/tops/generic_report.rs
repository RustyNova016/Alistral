use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;
use crate::utils::extensions::sequelles::hashjoin::HashJoin;

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
        create_top_table(&mut out, current_entity, previous_entity).await;
    }

    out
}

async fn create_top_table<Ent, Lis>(
    out: &mut String,
    current_recordings: &EntityWithListensCollection<Ent, Lis>,
    previous_recordings: &EntityWithListensCollection<Ent, Lis>,
) where
    Ent: HasRowID
        + MusicbrainzEntity
        + Clone
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
    Lis: ListenCollectionReadable + Default,
    EntityWithListens<Ent, Lis>: ListenCollWithTime + Clone,
{
    writeln!(out).unwrap();
    writeln!(out, "Here's the top 20 {}s:", Ent::entity_name()).unwrap();

    let join = current_recordings.0.hash_join(&previous_recordings.0);
    let stats = join
        .into_values()
        .map(|(current, previous)| {
            EntityListensComparison::new(current.cloned(), previous.cloned())
        })
        .collect_vec();

    let table_string = {
        //if self.listen_counts {
        let table: TopTable<TopListenDurCountRow<Ent>, OrderTableByListenDuration> =
            TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
        table.format(20, 0).await
    }; //else {
    //     let table: TopTable<TopListenDurationRow<_>, OrderTableByListenDuration> =
    //         TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
    //     table.format(20, 0).await
    // };
    writeln!(out, "{table_string}").unwrap();
}

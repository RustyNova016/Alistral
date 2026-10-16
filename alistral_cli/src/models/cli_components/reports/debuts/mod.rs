use core::fmt::Write as _;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::traits::mergable::Mergable;
use chrono::DateTime;
use chrono::Utc;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::shared_traits::debuted_on::DebutedOn;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::debuts::debuted_counts::discovery_counts;
use crate::models::cli_components::reports::debuts::debuted_duration::discovery_duration;
use crate::models::cli_components::reports::debuts::debuted_listens::discovery_listens;
use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;
use crate::models::client::AlistralCliClient;

pub mod debuted_counts;
pub mod debuted_duration;
pub mod debuted_listens;
pub mod no_debuts_with_previous;

pub async fn debut_report<Ent, Lis>(
    client: &AlistralCliClient,

    all_stats: &EntityWithListensCollection<Ent, Lis>,
    current_stats: &EntityWithListensCollection<Ent, Lis>,
    previous_stats: &EntityWithListensCollection<Ent, Lis>,

    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    previous_start_time: DateTime<Utc>,
) -> Result<String, ()>
where
    Ent: MusicbrainzEntity + DebutedOn,
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
    EntityWithListensCollection<Ent, Lis>:
        ListenCollectionReadable + Clone,
    // Table Bounds
    TopListenDurCountRow<Ent>: TableRow + From<EntityWithListens<Ent, Lis>>,
    OrderTableByListenDuration: TableSort<TopListenDurCountRow<Ent>>,
{
    let mut out = String::new();
    writeln!(
        out,
        "{}",
        Heading1(format!("Debuted {} 🌱", Ent::entity_name()))
    )
    .unwrap();
    writeln!(out).unwrap();

    let current_discoveries =
        get_debuted_entities(client, all_stats, &current_stats, start_time, end_time).await;
    let previous_discoveries = get_debuted_entities(
        client,
        all_stats,
        &previous_stats,
        previous_start_time,
        start_time,
    )
    .await;

    match (
        current_discoveries.is_empty(),
        previous_discoveries.is_empty(),
    ) {
        (true, true) => {
            writeln!(out, "No {} have debuted in this period", Ent::entity_name()).unwrap();
        }
        (true, false) => {
            writeln!(
                out,
                "{}",
                no_debuts_with_previous::no_discoveries_with_previous(&previous_discoveries)
            )
            .unwrap();
        }
        (false, true) | (false, false) => {
            writeln!(
                out,
                "{}",
                discovery_counts(
                    &current_discoveries,
                    &previous_discoveries,
                    &current_stats,
                    &previous_stats
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                discovery_listens(
                    &current_discoveries,
                    &previous_discoveries,
                    &current_stats,
                    &previous_stats
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                discovery_duration(
                    current_discoveries.clone(),
                    previous_discoveries,
                    current_stats.clone(),
                    previous_stats.clone()
                )
            )
            .unwrap();
            writeln!(out, "").unwrap();

            let table: TopTable<TopListenDurCountRow<_>, OrderTableByListenDuration> =
                TopTable::from_entity_listens(
                    current_discoveries,
                    OrderTableByListenDuration,
                    true,
                );
            writeln!(out, "{}", table.format(20, 0).await).unwrap();
        }
    }

    Ok(out)
}

async fn get_debuted_entities<Ent, Lis>(
    client: &AlistralCliClient,
    all_stats: &EntityWithListensCollection<Ent, Lis>,
    timeframe_stats: &EntityWithListensCollection<Ent, Lis>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> EntityWithListensCollection<Ent, Lis>
where
    Ent: DebutedOn,
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
{
    let mut out = EntityWithListensCollection::new();

    for entity_stats in timeframe_stats.iter() {
        let Some(debut_date) = entity_stats
            .entity()
            .debuted_on(&client.musicbrainz_db)
            .await
            .unwrap()
        else {
            continue;
        };

        if start_time <= debut_date && debut_date <= end_time {
            out.insert_or_merge_entity_stats(entity_stats.clone());
        }
    }

    out
}

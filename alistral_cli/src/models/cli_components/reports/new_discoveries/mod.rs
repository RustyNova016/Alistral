use core::fmt::Write as _;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::listen_with_time::iterator::IntoListenWithDurationIterator;
use alistral_core::traits::mergable::Mergable;
use chrono::DateTime;
use chrono::Utc;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::new_discoveries::discovered_counts::discovery_counts;
use crate::models::cli_components::reports::new_discoveries::discovered_duration::discovery_duration;
use crate::models::cli_components::reports::new_discoveries::discovered_listens::discovery_listens;
use crate::models::cli_components::reports::new_discoveries::no_discoveries_with_previous::no_discoveries_with_previous;
use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;

pub mod discovered_counts;
pub mod discovered_duration;
pub mod discovered_listens;
pub mod no_discoveries_with_previous;

pub async fn discoveries_report<Ent, Lis>(
    all_stats: &EntityWithListensCollection<Ent, Lis>,
    current_stats: &EntityWithListensCollection<Ent, Lis>,
    previous_stats: &EntityWithListensCollection<Ent, Lis>,

    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    previous_start_time: DateTime<Utc>,
) -> Result<String, ()>
where
    Ent: MusicbrainzEntity,
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
    EntityWithListensCollection<Ent, Lis>: ListenCollectionReadable + IntoListenWithDurationIterator + Clone,
    // Table Bounds
    TopListenDurCountRow<Ent>: TableRow + From<EntityWithListens<Ent, Lis>>,
    OrderTableByListenDuration: TableSort<TopListenDurCountRow<Ent>>,
{
    let mut out = String::new();
    writeln!(
        out,
        "{}",
        Heading1(format!("Discovered {} 🔎", Ent::entity_name()))
    )
    .unwrap();
    writeln!(out).unwrap();

    let current_discoveries =
        get_discovered_recordings(all_stats, &current_stats, start_time, end_time);
    let previous_discoveries =
        get_discovered_recordings(all_stats, &previous_stats, previous_start_time, start_time);

    match (
        current_discoveries.is_empty(),
        previous_discoveries.is_empty(),
    ) {
        (true, true) => {
            writeln!(out, "No new tracks have been discovered").unwrap();
        }
        (true, false) => {
            writeln!(
                out,
                "{}",
                no_discoveries_with_previous(&previous_discoveries)
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

fn get_discovered_recordings<Ent, Lis>(
    all_stats: &EntityWithListensCollection<Ent, Lis>,
    timeframe_stats: &EntityWithListensCollection<Ent, Lis>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> EntityWithListensCollection<Ent, Lis>
where
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
{
    let mut discovered = EntityWithListensCollection::new();

    for recording in all_stats.iter() {
        let Some(date_discovered) = recording.oldest_listen_date() else {
            continue;
        };

        if !(start_time <= date_discovered && date_discovered <= end_time) {
            continue;
        }

        let Some(new_recording) = timeframe_stats.get_by_id(recording.rowid()) else {
            continue;
        };

        discovered.insert_or_merge_entity_stats(new_recording.clone())
    }

    discovered
}

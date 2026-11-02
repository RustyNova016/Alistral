use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::traits::mergable::Mergable;
use convert_case::Casing;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::top_comeback::comeback_counts::comeback_counts;
use crate::models::cli_components::reports::top_comeback::comeback_duration::comeback_duration;
use crate::models::cli_components::reports::top_comeback::comeback_listens::comeback_listens;
use crate::models::cli_components::reports::top_comeback::no_comebacks_with_previous::no_comebacks_with_previous;
use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::table::TopTable;
use crate::models::timeframe::TimeFrame;

pub mod comeback_counts;
pub mod comeback_duration;
pub mod comeback_listens;
pub mod no_comebacks_with_previous;

pub async fn top_comeback_entities<Ent, Lis>(
    all_stats: &EntityWithListensCollection<Ent, Lis>,
    current_stats: &EntityWithListensCollection<Ent, Lis>,
    previous_stats: &EntityWithListensCollection<Ent, Lis>,

    // Settings
    time_frame: &TimeFrame,
) -> Result<String, ()>
where
    Ent: MusicbrainzEntity,
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
    EntityWithListensCollection<Ent, Lis>: ListenCollectionReadable + Clone,
    // Table Bounds
    TopListenDurCountRow<Ent>: TableRow + From<EntityWithListens<Ent, Lis>>,
    OrderTableByListenDuration: TableSort<TopListenDurCountRow<Ent>>,
{
    let mut out = String::new();
    writeln!(
        out,
        "{}",
        Heading1(
            format!("{} comebacks 🚀", Ent::entity_name()).to_case(convert_case::Case::Sentence)
        )
    )
    .unwrap();
    writeln!(out).unwrap();

    let current_comebacks = get_comeback_entities(all_stats, current_stats, time_frame);
    let previous_comebacks =
        get_comeback_entities(all_stats, previous_stats, &time_frame.previous());

    match (current_comebacks.is_empty(), previous_comebacks.is_empty()) {
        (true, true) => {
            writeln!(out, "No {} have made their comeback", Ent::entity_name()).unwrap();
        }
        (true, false) => {
            writeln!(out, "{}", no_comebacks_with_previous(&previous_comebacks)).unwrap();
        }
        (false, true) | (false, false) => {
            writeln!(
                out,
                "{}",
                comeback_counts(
                    &current_comebacks,
                    &previous_comebacks,
                    &current_stats,
                    &previous_stats
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                comeback_listens(
                    &current_comebacks,
                    &previous_comebacks,
                    &current_stats,
                    &previous_stats
                )
            )
            .unwrap();
            writeln!(
                out,
                "{}",
                comeback_duration(
                    current_comebacks.clone(),
                    previous_comebacks,
                    current_stats.clone(),
                    previous_stats.clone()
                )
            )
            .unwrap();
            writeln!(out, "").unwrap();

            let table: TopTable<TopListenDurCountRow<_>, OrderTableByListenDuration> =
                TopTable::from_entity_listens(current_comebacks, OrderTableByListenDuration, true);
            writeln!(out, "{}", table.format(20, 0).await).unwrap();
        }
    }

    Ok(out)
}

fn get_comeback_entities<Ent, Lis>(
    all_stats: &EntityWithListensCollection<Ent, Lis>,
    timeframe_stats: &EntityWithListensCollection<Ent, Lis>,
    time_frame: &TimeFrame,
) -> EntityWithListensCollection<Ent, Lis>
where
    EntityWithListens<Ent, Lis>: Clone + ListenCollectionReadable + HasRowID + Mergable,
{
    let mut comeback = EntityWithListensCollection::new();

    let previous_tf = time_frame.previous();

    for recording in all_stats.iter() {
        let Some(date_discovered) = recording.oldest_listen_date() else {
            continue;
        };

        // We make sure the entity is discovered before the previous timeframe's started
        if previous_tf.start < date_discovered {
            continue;
        }

        // Don't take entities listened in the last time frame
        if recording
            .iter_unique_listens()
            .any(|lis| previous_tf.in_timespan(lis.listened_at_as_datetime()))
        {
            continue;
        }

        let Some(new_recording) = timeframe_stats.get_by_id(recording.rowid()) else {
            continue;
        };

        comeback.insert_or_merge_entity_stats(new_recording.clone())
    }

    comeback
}

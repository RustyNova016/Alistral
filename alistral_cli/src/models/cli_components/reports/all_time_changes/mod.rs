use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::utils::display_table::create_report_top_table;

pub async fn all_time_changes<Ent, Lis>(
    current_all_time: &EntityWithListensCollection<Ent, Lis>,
    previous_all_time: &EntityWithListensCollection<Ent, Lis>,
    // Settings
) -> String
where
    Ent: HasRowID
        + MusicbrainzEntity
        + Clone
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
    Lis: ListenCollectionReadable + Default,
    EntityWithListens<Ent, Lis>: ListenCollectionReadable + Clone,
{
    let mut out = String::new();
    writeln!(
        out,
        "{}",
        Heading1(format!(
            "All time top changes for {}s 📈",
            Ent::entity_name()
        ))
    )
    .unwrap();
    writeln!(out, "Here is the changes made to the all time top 20:").unwrap();
    writeln!(out).unwrap();

    // === Stats lines ===
    if !current_all_time.is_empty() || !previous_all_time.is_empty() {
        create_report_top_table(
            &mut out,
            current_all_time,
            previous_all_time,
            &format!("{}s", Ent::entity_name()),
        )
        .await;
    }

    out
}

use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools;
use tuillez::extensions::chrono_exts::DurationExt as _;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_recording(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups =
        RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT.core, listens)
            .await
            .expect("Error while fetching recordings")
            .into_iter()
            .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        println!(
            "[{}] {}",
            group.listen_count(),
            group
                .entity()
                .pretty_format_with_credits(conn, &ALISTRAL_CLIENT.musicbrainz_db, true)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

pub async fn stats_recording_time(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups =
        RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT.core, listens)
            .await
            .expect("Error while fetching recordings")
            .into_iter()
            .collect_vec();
    groups.sort_by_key(|a| Reverse(a.get_time_listened()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        println!(
            "[{}] {}",
            group
                .get_time_listened()
                .map(|dur| dur.format_hh_mm())
                .unwrap_or_else(|| "??".to_string()),
            group
                .entity()
                .pretty_format_with_credits(conn, &ALISTRAL_CLIENT.musicbrainz_db, true)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli_paging::CLIPager;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub async fn stats_artist(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups =
        ArtistWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT.core, listens)
            .await
            .expect("Error while fetching recordings")
            .into_iter()
            .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        group
            .entity()
            .fetch_if_incomplete(conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await
            .expect("Error while fetching release");
        println!(
            "[{}] {}",
            group.listen_count(),
            group
                .entity()
                .format_with_async(&LISTENBRAINZ_FMT)
                .await
                .expect("Couldn't format entity")
        );

        if !pager.inc() {
            break;
        }
    }
}

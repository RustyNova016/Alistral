use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::release_strategy;
use crate::utils::cli_paging::CLIPager;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub async fn stats_releases(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = ReleaseWithRecordingsCollection::from_listencollection(
        listens,
        &release_strategy(&ALISTRAL_CLIENT),
    )
    .await
    .expect("Error while fetching releases")
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
                .expect("Error getting formated release name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

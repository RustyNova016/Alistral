use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::release_group_strategy;
use crate::utils::cli_paging::CLIPager;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub async fn stats_release_groups(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = ReleaseGroupWithListensCollection::from_listencollection(
        listens,
        &release_group_strategy(&ALISTRAL_CLIENT),
    )
    .await
    .expect("Error while fetching release groups")
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

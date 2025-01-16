use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::datastructures::listen_collection::ListenCollection;
use itertools::Itertools;

use crate::utils::cli::display::ReleaseExt as _;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_releases(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = ReleaseWithListensCollection::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_iter()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        group
            .entity()
            .fetch_if_incomplete(conn)
            .await
            .expect("Error while fetching release");
        println!(
            "[{}] {}",
            group.listen_count(),
            group
                .entity()
                .pretty_format_with_credits(conn, true)
                .await
                .expect("Error getting formated release name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

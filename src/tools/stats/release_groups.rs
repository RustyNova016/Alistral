use core::cmp::Reverse;

use itertools::Itertools;

use crate::datastructures::entity_with_listens::release_group_with_listens::ReleaseGroupWithListens;
use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli::display::ReleaseGroupExt as _;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_release_groups(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = ReleaseGroupWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        group
            .release_group()
            .fetch_if_incomplete(conn)
            .await
            .expect("Error while fetching release");
        println!(
            "[{}] {}",
            group.listen_count(),
            group
                .release_group()
                .pretty_format_with_credits(conn, true)
                .await
                .expect("Error getting formated release name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

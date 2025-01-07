use core::cmp::Reverse;

use itertools::Itertools;

use crate::datastructures::entity_with_listens::work_with_listens::WorkWithRecordingListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli::display::WorkExt as _;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_works(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = WorkWithRecordingListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.len()));

    let mut pager = CLIPager::new(10);

    if groups.is_empty() {
        println!("No works have been found");
    }

    for group in groups {
        println!(
            "[{}] {}",
            group.len(),
            group
                .work()
                .pretty_format()
                .await
                .expect("Couldn't format the work")
        );

        if !pager.inc() {
            break;
        }
    }
}

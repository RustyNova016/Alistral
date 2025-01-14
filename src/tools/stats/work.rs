use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::work::WorkWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::datastructures::listen_collection::ListenCollection;
use itertools::Itertools;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::entity_with_listens::work_with_listens::WorkWithRecordingListens;
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

pub async fn stats_works_recursive(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let recordings = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings");

    let mut groups = WorkWithListens::from_recording_with_listens(conn, recordings)
        .await
        .expect("Error while fetching works");

    groups
        .add_parents_recursive(conn)
        .await
        .expect("Couldn't add parents");

    let mut as_vec = groups.0.into_values().collect_vec();
    as_vec.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    if as_vec.is_empty() {
        println!("No works have been found");
    }

    for group in as_vec {
        println!(
            "[{}] {}",
            group.listen_count(),
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

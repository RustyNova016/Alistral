use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::work_strategy;
use crate::utils::cli_paging::CLIPager;

pub async fn stats_works(_conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups =
        WorkWithListensCollection::from_listencollection(listens, &work_strategy(&ALISTRAL_CLIENT))
            .await
            .expect("Error while fetching works")
            .into_iter()
            .collect_vec();
    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    if groups.is_empty() {
        println!("No works have been found");
    }

    for group in groups {
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

pub async fn stats_works_recursive(_conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let strategy = work_strategy(&ALISTRAL_CLIENT).with_recursive_parents();

    let mut groups = WorkWithListensCollection::from_listencollection(listens, &strategy)
        .await
        .expect("Error while fetching works")
        .0
        .into_values()
        .collect_vec();

    groups.sort_by_key(|a| Reverse(a.listen_count()));

    let mut pager = CLIPager::new(10);

    if groups.is_empty() {
        println!("No works have been found");
    }

    for group in groups {
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

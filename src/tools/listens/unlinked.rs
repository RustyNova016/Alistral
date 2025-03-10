use std::cmp::Reverse;

use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use alistral_core::datastructures::entity_with_listens::messybrainz::collection::MessybrainzWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use tracing::info;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::SortSorterBy;
use crate::utils::cli_paging::CLIPager;

pub async fn unmapped_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    sort: Option<SortSorterBy>,
) {
    info!("Fetching unmapped for user {username}");
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Unmapped)
        .user(username.to_string())
        .build()
        .fetch(conn, &ALISTRAL_CLIENT.core)
        .await
        .expect("Couldn't fetch listens");

    let unlinkeds = MessybrainzWithListensCollection::from_listencollection(conn, listens)
        .await
        .expect("Couldn't associate the listen to their messybrainz data");
    //let unlinked_count = unlinkeds.listen_count();

    let mut messy_recordings = unlinkeds.iter().collect_vec();

    match sort.unwrap_or_default() {
        SortSorterBy::Name => {
            messy_recordings.sort_by_key(|messy_data| &messy_data.entity().recording);
        }

        SortSorterBy::Oldest => {
            messy_recordings.sort_by_key(|messy_data| {
                messy_data
                    .get_oldest_listen()
                    .map(|listen| listen.listened_at)
            });
        }

        SortSorterBy::Count => {
            messy_recordings.sort_by_key(|messy_data| Reverse(messy_data.listens().len()));
        }
    }

    println!("Done! Here are {username}'s top unmapped listens:");

    let mut pager = CLIPager::new(5);

    //println!("Total: {unlinked_count} unmapped recordings");
    for record in &messy_recordings {
        let pager_continue = pager.execute(|| {
            println!(
                "({}) {} - {}",
                record.listens().len(),
                record.entity().recording,
                record.entity().artist_credit
            );

            let latest_listen = record.get_latest_listen();

            println!(
                "    -> https://listenbrainz.org/user/{}/?min_ts={}&max_ts={}",
                username,
                latest_listen
                    .map(|listen| listen.listened_at - 1)
                    .unwrap_or(0),
                latest_listen
                    .map(|listen| listen.listened_at + 1)
                    .unwrap_or(0)
            );
            println!();
        });

        if !pager_continue {
            return;
        }
    }
}

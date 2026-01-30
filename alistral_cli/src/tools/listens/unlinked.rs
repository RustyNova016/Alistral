use std::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::messybrainz::collection::MessybrainzWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use clap::Parser;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::SortSorterBy;
use crate::utils::cli_paging::CLIPager;
use crate::utils::user_inputs::UserInputParser;

/// List all of your unlinked listens
///
/// This command will list all your unmapped listens, grouped by similarity.
/// It also gives a link to quickly look up the listen in listenbrainz, and go link it
///
/// ```text
///
/// (1) Paul's Dream (Dune) - Caster
///     -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>
///
/// (7) Raise Your Weapon - KLOUD
///     -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>
///
/// Total: 8 unlinked recordings
///
/// ```
///
/// > Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens
/// > by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
///
/// > This also means that the same recording can be shown twice in the list.
/// > For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.
#[derive(Parser, Debug, Clone)]
pub struct ListenUnlinkedCommand {
    /// Name of the user to fetch unlinked listen from
    username: Option<String>,

    /// Sort the listens by type
    #[arg(short, long)]
    sort: Option<SortSorterBy>,
}

impl ListenUnlinkedCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);
        let conn = &mut *ALISTRAL_CLIENT.get_conn().await;

        let listens = Listen::get_or_fetch_listens()
            .client(&ALISTRAL_CLIENT.musicbrainz_db)
            .incremental(true)
            .unmapped(true)
            .users(&[&username])
            .call()
            .await
            .unwrap();

        let unlinkeds = MessybrainzWithListensCollection::from_listencollection_default(
            conn,
            ListenCollection::new(listens),
        )
        .await
        .expect("Couldn't associate the listen to their messybrainz data");
        //let unlinked_count = unlinkeds.listen_count();

        let mut messy_recordings = unlinkeds.iter().collect_vec();

        match self.sort.unwrap_or_default() {
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
}

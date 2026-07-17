use alistral_core::datastructures::listen_collection::ListenCollection;
use clap::Parser;
use inquire::Select;
use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use sequelles::Delete as _;
use tuillez::OwoColorize as _;

use crate::ALISTRAL_CLIENT;
use crate::utils::user_inputs::UserInputParser;

/// Remove duplicate multiscrobbler listens
#[derive(Parser, Debug, Clone)]
pub struct UnstableMSDupesCommand {
    /// Reload the listens of this user
    pub username: Option<String>,
}

impl UnstableMSDupesCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);
        let listens = Listen::get_or_fetch_listens()
            .client(&ALISTRAL_CLIENT.musicbrainz_db)
            .incremental(true)
            .unmapped(true)
            .mapped(true)
            .users(&[&username])
            .call()
            .await
            .unwrap();

        let listens = ListenCollection::new(listens);
        let mut deleted_listens = Vec::new();

        for listen in listens.iter() {
            if !inspect_listen(listen, &listens, &mut deleted_listens).await {
                return;
            }
        }
    }
}

async fn inspect_listen(
    listen: &Listen,
    listens: &ListenCollection,
    deleted_listens: &mut Vec<Listen>,
) -> bool {
    let metadata = listen
        .listen_metadata()
        .unwrap_or_default()
        .unwrap_or_default();

    // Check if the listen was submited from MS
    if metadata.submission_client() != Some("multi-scrobbler")
        || metadata.music_service_name() != Some("multi-scrobbler")
    {
        return true;
    }

    // Check if there's a listen at the same time as us
    let other_listens = listens
        .find_by_timestamp(listen.listened_at)
        .filter(|l| l.id != listen.id)
        .filter(|l| deleted_listens.iter().all(|deleted| deleted.id != l.id))
        .collect_vec();

    if other_listens.is_empty() {
        return true;
    }

    let dupe_messy = listen
        .get_messybrainz_data(&mut ALISTRAL_CLIENT.musicbrainz_db.get_conn().await.unwrap())
        .await
        .unwrap();

    println!("Found multiscrobbler duplicate: ");
    println!("    Duplicate: ");
    println!("        track: {}", dupe_messy.recording);
    println!("        artist: {}", dupe_messy.artist_credit);
    println!(
        "        release: {}",
        dupe_messy.release.unwrap_or_default()
    );
    println!(
        "        Music service: {}",
        metadata
            .music_service_name()
            .unwrap_or("None")
            .to_string()
            .red()
    );
    for listen in other_listens {
        let messy = listen
            .get_messybrainz_data(&mut ALISTRAL_CLIENT.musicbrainz_db.get_conn().await.unwrap())
            .await
            .unwrap();

        let metadata = listen
            .listen_metadata()
            .unwrap_or_default()
            .unwrap_or_default();

        println!();
        println!("    Original: ");
        println!("        track: {}", messy.recording);
        println!("        artist: {}", messy.artist_credit);
        println!("        release: {}", messy.release.unwrap_or_default());
        println!(
            "        Music service: {}",
            metadata
                .music_service_name()
                .unwrap_or("None")
                .to_string()
                .green()
        );
    }

    println!(
        "    -> https://listenbrainz.org/user/{}/?min_ts={}&max_ts={}",
        listen.user,
        listen.listened_at - 1,
        listen.listened_at + 1
    );

    println!();

    let options = vec!["Keep", "Delete", "Stop"];
    let response = Select::new("", options).prompt().unwrap();

    match response {
        "Delete" => {
            println!("Deleting listen id {}", listen.id);

            ALISTRAL_CLIENT
                .listenbrainz
                .endpoints()
                .delete_listen()
                .listened_at(listen.listened_at.cast_unsigned())
                .recording_msid(&listen.recording_msid)
                .call()
                .unwrap()
                .send_async(ALISTRAL_CLIENT.listenbrainz.api_client())
                .await
                .unwrap();

            listen
                .delete(&mut *ALISTRAL_CLIENT.musicbrainz_db.get_conn().await.unwrap())
                .await
                .unwrap();

            deleted_listens.push(listen.to_owned());
            true
        }
        "Keep" => true,
        _ => false,
    }
}

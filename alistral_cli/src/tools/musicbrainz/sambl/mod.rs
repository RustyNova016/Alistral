use std::sync::Arc;

use clap::Parser;
use futures::SinkExt;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::join;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::FetchAndSave;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::main_entities::crawler::crawler;
use streamies::TryStreamies;

use crate::ALISTRAL_CLIENT;
use crate::tools::musicbrainz::clippy::crawler_poller;
use crate::tools::musicbrainz::clippy::sambl_check::samble_clippy_poller;
use crate::tools::musicbrainz::clippy::sambl_check::samble_clippy_stream;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

#[derive(Parser, Debug, Clone)]
/// Search for potential mistakes, missing data and style issues. This allows to quickly pin down errors that can be corrected
///
/// ⚠️ All tips are suggestions. Take them with a grain of salt. If you are unsure, it's preferable to skip.
pub struct MusicbrainzSamblCommand {
    /// The MBID of a recording to start from
    pub start_artists: Vec<String>,

    /// Sort the initial artists by their name
    #[arg(short, long)]
    pub sort: bool,
}

impl MusicbrainzSamblCommand {
    pub async fn run(&self) {
        let nodes = self.get_start_artists().await;

        // Prepare clippys:
        let black_list = WhitelistBlacklist::BlackList(Vec::new());
        let (sambl_send, sambl_stream) = samble_clippy_stream(&black_list);

        let crawler = crawler(ALISTRAL_CLIENT.musicbrainz_db.clone(), nodes)
            .map_ok(|entity| {
                let mut sambl_send = sambl_send.clone();
                async move {
                    sambl_send.send(entity.clone()).await.unwrap();
                }
            })
            .extract_future_ok()
            .buffered(16)
            .map_err(crate::Error::from);

        let _ = join!(crawler_poller(crawler), samble_clippy_poller(sambl_stream));

        println!("No more data to process");
    }

    async fn get_start_artists(&self) -> Vec<Arc<MainEntity>> {
        let mut results = Vec::new();

        for artist in &self.start_artists {
            let mbid = read_mbid_from_input(&artist).expect("Couldn't read mbid");

            let artist =
                Artist::fetch_and_save_as_task(ALISTRAL_CLIENT.musicbrainz_db.clone(), &mbid)
                    .await
                    .unwrap()
                    .expect("Couldn't find artist");

            results.push(Arc::new(MainEntity::Artist(artist)));
        }

        results
    }
}

use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use clap::Parser;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;
use tuillez::fatal_error::IntoFatal;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::config::Config;
use crate::utils::cli::parsing::assert_recording_mbid;

#[cfg(not(test))]
use crate::utils::cli::await_next;

#[derive(Parser, Clone, Debug)]
pub(super) struct LookupRecordingCommand {
    /// The id of the entity (Accept URLs)
    id: String,

    /// Get the data of this user
    username: Option<String>,
}

impl LookupRecordingCommand {
    pub async fn run(&self) {
        let conn = &mut *ALISTRAL_CLIENT
            .musicbrainz_db
            .get_raw_connection()
            .await
            .expect("Couldn't connect to the database");
        let username = Config::check_username(&self.username);
        let id = assert_recording_mbid(conn, &self.id).await;

        // Fetch the listens.
        let listens = ListenFetchQuery::builder()
            .fetch_recordings_redirects(true)
            .returns(ListenFetchQueryReturn::Mapped)
            .user(username.to_string())
            .build()
            .fetch(conn, &ALISTRAL_CLIENT.core)
            .await
            .expect_fatal("Couldn't fetch listens")
            .unwrap();

        // Refetch the recording to make sure it's up to date
        let Some(recording) = Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, &id)
            .await
            .map_err(crate::Error::from)
            .unwrap()
        else {
            info!("Couldn't find the recording with id: {id}");
            return;
        };

        let mut all_listens = RecordingWithListens::from_listencollection(conn, listens)
            .await
            .expect("Couldn't load recordings");

        let target_recording = all_listens.get_or_new(recording).clone();

        println!(
            "{}",
            target_recording
                .get_lookup_report(conn, &all_listens)
                .await
                .expect("Couldn't generate lookup report")
        );

        #[cfg(not(test))]
        await_next();
    }
}

use chrono::Utc;
use clap::Parser;
use musicbrainz_db_lite::api::listenbrainz::listen::fetching::query::ListenFetchAPIQuery;
use tuillez::fatal_error::IntoFatal;

use crate::ALISTRAL_CLIENT;

#[derive(Parser, Debug, Clone)]
pub struct ListenReloadCommand {
    /// Reload the listens of this user
    pub user: Option<String>,
}

impl ListenReloadCommand {
    pub async fn run(&self) {
        let mut query = ListenFetchAPIQuery::try_new(
            ALISTRAL_CLIENT
                .config
                .get_username_or_panic(self.user.clone()),
            None,
            Some(Utc::now()),
            1000,
        )
        .unwrap();

        query
            .request_and_save(&ALISTRAL_CLIENT.musicbrainz_db)
            .await
            .expect_fatal("Couldn't reload the listens")
            .unwrap();
    }
}

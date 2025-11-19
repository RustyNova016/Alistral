use clap::Parser;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::ALISTRAL_CLIENT;
use crate::utils::user_inputs::UserInputParser;

#[derive(Parser, Debug, Clone)]
pub struct ListenReloadCommand {
    /// Reload the listens of this user
    pub username: Option<String>,
}

impl ListenReloadCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);
        Listen::fetch_and_save_full(&ALISTRAL_CLIENT.musicbrainz_db, &username)
            .await
            .unwrap();
    }
}

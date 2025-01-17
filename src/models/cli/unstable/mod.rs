// use crate::models::config::Config;
//use crate::tools::unstable::best_of::best_of_checker;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct UnstableCommand {
    #[command(subcommand)]
    pub command: UnstableSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UnstableSubcommands {
    // /// See what your favourite Monstercat releases of this year are,
    // /// and have an easier time voting for this year's Best of 2024!
    // ///
    // /// You can get a listen dump [here](https://listenbrainz.org/settings/export/)
    // BestOfMC {
    //     /// Name of the user to look up stats from
    //     username: Option<String>,
    // },
}

impl UnstableSubcommands {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) {
        // match self {
        //     Self::BestOfMC { username } => {
        //         let username = Config::check_username(username);
        //         best_of_checker(conn, &username).await;
        //     }
        // }
    }
}

use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "clippy")]
use crate::tools::musicbrainz::clippy::MusicbrainzClippyCommand;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct MusicbrainzCommand {
    #[command(subcommand)]
    subcommand: MusicbrainzSubcommands,
}

impl MusicbrainzCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) {
        self.subcommand.run(conn).await;
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MusicbrainzSubcommands {
    #[cfg(feature = "clippy")]
    Clippy(MusicbrainzClippyCommand),
}

impl MusicbrainzSubcommands {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) {
        #[cfg(feature = "clippy")]
        match self {
            #[cfg(feature = "clippy")]
            Self::Clippy(val) => val.run().await,
        }
    }
}

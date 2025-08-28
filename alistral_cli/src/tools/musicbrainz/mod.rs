use clap::Parser;
use clap::Subcommand;

use crate::tools::musicbrainz::clippy::MusicbrainzClippyCommand;
use crate::tools::musicbrainz::sambl::MusicbrainzSamblCommand;

pub mod clippy;
pub mod sambl;

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
    Clippy(MusicbrainzClippyCommand),
    Sambl(MusicbrainzSamblCommand),
}

impl MusicbrainzSubcommands {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) {
        match self {
            Self::Clippy(val) => val.run().await,
            Self::Sambl(val) => val.run().await,
        }
    }
}

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

#[derive(Subcommand, Debug, Clone)]
pub enum MusicbrainzSubcommands {
    Clippy(MusicbrainzClippyCommand),
    Sambl(MusicbrainzSamblCommand),
}

impl MusicbrainzCommand {
    pub async fn run(&self) {
        match &self.subcommand {
            MusicbrainzSubcommands::Clippy(val) => val.run().await,
            MusicbrainzSubcommands::Sambl(val) => val.run().await,
        }
    }
}

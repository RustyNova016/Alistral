use clap::ValueEnum;
use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

use crate::tools::playlist::convert::PlaylistConvertCommand;

pub mod convert;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct PlaylistCommand {
    #[command(subcommand)]
    pub command: PlaylistSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlaylistSubcommands {
    /// Convert a playlist from one service to another
    Convert(PlaylistConvertCommand),
}

impl PlaylistCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        match &self.command {
            PlaylistSubcommands::Convert(val) => {
                val.run().await?;
            }
        }

        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PlaylistOrigin {
    Listenbrainz,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PlaylistTarget {
    #[cfg(feature = "youtube")]
    Youtube,
    #[cfg(feature = "subsonic")]
    Subsonic,
}

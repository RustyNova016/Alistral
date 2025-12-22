use clap::ValueEnum;
use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

#[cfg(any(feature = "youtube"))]
use crate::tools::playlist::convert::PlaylistConvertCommand;

#[cfg(any(feature = "youtube"))]
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
    #[cfg(any(feature = "youtube"))]
    Convert(PlaylistConvertCommand),
}

impl PlaylistCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        #[cfg(any(feature = "youtube"))]
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
    Youtube,
}

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

impl PlaylistCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        self.command.run(conn).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlaylistSubcommands {
    /// Convert a playlist from one service to another
    Convert(PlaylistConvertCommand),
}

impl PlaylistSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::Convert(val) => {
                val.run(conn).await?;
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

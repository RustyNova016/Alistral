use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

use crate::tools::interzic::get_mapping::GetMappingCommand;
use crate::tools::interzic::overwrite::OverwriteCommand;
use crate::tools::interzic::reverse_mapping::ReverseMappingCommand;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct InterzicCommand {
    #[command(subcommand)]
    pub command: InterzicSubcommands,
}

impl InterzicCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        self.command.run(conn).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum InterzicSubcommands {
    /// Fetch the id of a recording on an external service
    GetMapping(GetMappingCommand),

    /// Get the recording mapped to this id
    ReverseMapping(ReverseMappingCommand),

    /// Overwrite a mapping for an user
    Overwrite(OverwriteCommand),
}

impl InterzicSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::GetMapping(args) => Ok(args.run(conn).await?),
            Self::ReverseMapping(args) => Ok(args.run(conn).await?),
            InterzicSubcommands::Overwrite(args) => Ok(args.run(conn).await?),
        }
    }
}

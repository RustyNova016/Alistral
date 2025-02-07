use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

use crate::tools::interzic::get_mapping::GetMappingCommand;

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
    GetMapping(GetMappingCommand),
}

impl InterzicSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::GetMapping(args) => Ok(args.run(conn).await?),
        }
    }
}

//pub mod best_of;

use clap::Parser;

use clap::Subcommand;
use tuillez::fatal_error::FatalError;

use crate::tools::unstable::where_like::WhereLikeCommand;

pub mod where_like;
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct UnstableCommand {
    #[command(subcommand)]
    pub command: UnstableSubcommands,
}

impl UnstableCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        self.command.run(conn).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum UnstableSubcommands {
    WhereLike(WhereLikeCommand),
}

impl UnstableSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::WhereLike(args) => Ok(args.run(conn).await?)
        }
    }
}



use clap::Parser;
use clap::Subcommand;

use crate::tools::unstable::multiscrobbler_dupes::UnstableMSDupesCommand;

pub mod multiscrobbler_dupes;
//pub mod best_of;

/// Commands that aren't polished enough or may change a lot
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct UnstableCommand {
    #[command(subcommand)]
    pub command: UnstableSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UnstableSubcommands {
    Duplicates(UnstableMSDupesCommand),
}

impl UnstableCommand {
    pub async fn run(&self) {
        match &self.command {
            UnstableSubcommands::Duplicates(val) => val.run().await,
        }
    }
}

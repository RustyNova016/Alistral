use clap::Parser;
use clap::Subcommand;

use crate::tools::stats::tops::StatsTopCommand;

pub mod tops;

/// All kinds of statistics about your listening habits
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct StatsCommand {
    #[command(subcommand)]
    pub command: StatsSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum StatsSubcommands {
    Top(StatsTopCommand),
}

impl StatsCommand {
    pub async fn run(&self) {
        match &self.command {
            StatsSubcommands::Top(val) => val.run().await,
        }
    }
}

use clap::Parser;
use clap::Subcommand;

use crate::tools::stats::tops::StatsTopCommand;
use crate::tools::stats::year_in_music::StatsYIMCommand;

pub mod tops;
pub mod year_in_music;

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
    Yim(StatsYIMCommand),
}

impl StatsCommand {
    pub async fn run(&self) {
        match &self.command {
            StatsSubcommands::Top(val) => val.run().await,
            StatsSubcommands::Yim(val) => val.run().await,
        }
    }
}

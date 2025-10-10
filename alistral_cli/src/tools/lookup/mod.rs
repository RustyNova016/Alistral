use clap::Parser;
use clap::Subcommand;

use crate::tools::lookup::recording::LookupRecordingCommand;
use crate::tools::lookup::user::LookupUserCommand;

pub mod lookup_components;
pub mod recording;
pub mod user;

#[derive(Parser, Clone, Debug)]
pub struct LookupCommand {
    #[command(subcommand)]
    command: LookupSubcommands,
}

#[derive(Subcommand, Clone, Debug)]
enum LookupSubcommands {
    User(LookupUserCommand),
    Recording(LookupRecordingCommand),
}

impl LookupCommand {
    pub async fn run(&self) {
        match &self.command {
            LookupSubcommands::User(cmd) => cmd.run().await,
            LookupSubcommands::Recording(cmd) => cmd.run().await,
        }
    }
}

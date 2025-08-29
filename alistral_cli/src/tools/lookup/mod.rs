use crate::tools::lookup::recording::LookupRecordingCommand;
use crate::tools::lookup::user::LookupUserCommand;
use clap::Parser;
use clap::Subcommand;

pub mod components;
pub mod recording;
pub mod user;

#[derive(Parser, Clone, Debug)]
pub struct LookupCommand {
    #[command(subcommand)]
    command: LookupSubcommands,
}

impl LookupCommand {
    pub async fn run(&self) {
        self.command.run().await
    }
}

#[derive(Subcommand, Clone, Debug)]
enum LookupSubcommands {
    User(LookupUserCommand),
    Recording(LookupRecordingCommand),
}

impl LookupSubcommands {
    pub async fn run(&self) {
        match self {
            Self::User(cmd) => cmd.run().await,
            Self::Recording(cmd) => cmd.run().await,
        }
    }
}

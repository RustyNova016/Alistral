pub mod recording_b;
pub mod components;
pub mod recording;
pub mod user;
use crate::models::cli::lookup::LookupTarget;
use crate::tools::lookup::recording_b::LookupRecordingCommand;
use crate::tools::lookup::user::LookupUserCommand;
use clap::Parser;
use clap::Subcommand;
use recording::lookup_recording;
use tuillez::fatal_error::FatalError;

pub async fn lookup_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    id: &str,
    target: LookupTarget,
) -> Result<(), FatalError> {
    match target {
        LookupTarget::Recording => lookup_recording(conn, username, id).await,
    }
}

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
    Recording(LookupRecordingCommand),
    User(LookupUserCommand),
}

impl LookupSubcommands {
    pub async fn run(&self) {
        match self {
            Self::Recording(cmd) => cmd.run().await,
            Self::User(cmd) => cmd.run().await,
        }
    }
}

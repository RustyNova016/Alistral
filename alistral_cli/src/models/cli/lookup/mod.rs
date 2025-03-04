use crate::models::config::Config;
use crate::tools::lookup::lookup_command;
use crate::utils::cli::parsing::assert_recording_mbid;
use clap::Parser;
use clap::ValueEnum;
use tuillez::fatal_error::FatalError;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct LookupCommand {
    /// The type of entity to look for
    entity_type: LookupTarget,

    /// The id of the entity (Accept URLs)
    id: String,

    /// Name of the user to look up stats from
    username: Option<String>,
}

impl LookupCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        let id = assert_recording_mbid(conn, &self.id).await;

        lookup_command(
            conn,
            &Config::check_username(&self.username),
            &id,
            self.entity_type,
        )
        .await?;
        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum LookupTarget {
    Recording,
}

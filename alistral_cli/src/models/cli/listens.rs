use clap::Parser;
use clap::Subcommand;

use crate::models::config::Config;
use crate::tools::listens::clear::ListensClearCommand;
use crate::tools::listens::import::ListenImportDumpCommand;
use crate::tools::listens::mapper::listen_mapper_convert_mbids;
use crate::tools::listens::reload::ListenReloadCommand;
use crate::tools::listens::submit::ListenSubmitCommand;
use crate::tools::listens::wrong_mapping::wrong_mapping;
use crate::utils::cli::read_mbid_from_input;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ListenCommand {
    #[command(subcommand)]
    subcommand: ListenSubcommands,
}

impl ListenCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        self.subcommand.run(conn).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListenSubcommands {
    Clear(ListensClearCommand),
    ImportDump(ListenImportDumpCommand),

    /// Changes all the listens of a recording into another. Useful if LB mapped to a recording you never listened
    RemapMsid {
        /// The MBID of the recording
        original_id: String,

        /// The MBID of the recorind to replace it with
        new_id: String,

        /// Your username
        username: Option<String>,

        /// Your account token
        token: Option<String>,
    },

    Reload(ListenReloadCommand),

    Submit(ListenSubmitCommand),

    WrongMapping {
        /// Your username
        username: Option<String>,
    },
}

impl ListenSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        match self {
            Self::Clear(val) => val.run().await,
            Self::ImportDump(val) => val.run().await,
            Self::RemapMsid {
                original_id,
                new_id,
                username,
                token,
            } => {
                listen_mapper_convert_mbids(
                    conn,
                    &read_mbid_from_input(original_id)
                        .expect("Couldn't read `original_id` as MBID"),
                    &read_mbid_from_input(new_id).expect("Couldn't read `new_id` as MBID"),
                    &Config::check_username(username),
                    &Config::check_token(&Config::check_username(username), token),
                )
                .await;
            }
            Self::Reload(cmd) => cmd.run().await?,
            Self::Submit(cmd) => cmd.run().await?,
            Self::WrongMapping { username } => {
                wrong_mapping(conn, Config::check_username(username)).await;
            }
        }

        Ok(())
    }
}

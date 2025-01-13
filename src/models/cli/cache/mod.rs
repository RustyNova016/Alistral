use std::path::PathBuf;

use crate::database::get_conn;
use crate::database::DB_LOCATION;
use crate::models::config::Config;
//use crate::models::config::Config;
use crate::tools::cache::copy_to_debug;
use crate::tools::cache::delete_database;
use crate::tools::cache::refresh_data::refresh_data;
//use crate::tools::listens::import::import_listen_dump;
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use color_eyre::owo_colors::OwoColorize;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CacheSubcommands {
    /// Copy the release database to the debug one.
    ///
    /// ⚠️ This wipe the debug database.
    ///
    /// ⚠️ If there is migrations, do `cargo sqlx migrate run` next
    #[cfg(debug_assertions)]
    CopyToDebug,

    /// Wipe the cache's data
    ///
    /// This is useful if you need disk space, or need to manually rebuild in case of corruption
    Clear,

    /// Clear all the listens from the database
    ClearListens {
        /// Only delete listens of user
        user: Option<String>,
    },

    /// Initialise the database.
    InitDatabase {
        /// Wipe the database file beforehand
        #[arg(long)]
        reset: bool,
    },

    /// Load a listen dump from the website
    ///
    /// Allows to load an exported dump of you listens. This is often faster than using the app.
    /// This also prevent stumbling into LB-1584
    ///
    /// You can get a listen dump [here](https://listenbrainz.org/settings/export/)
    LoadDump {
        /// Path to the dump file
        path: PathBuf,

        /// Name of the user to import those listens for
        username: Option<String>,
    },

    RefreshData {
        /// Name of the user to refresh the data
        #[arg(short, long)]
        username: Option<String>,

        /// How many entities to refresh
        #[arg(short, long)]
        limit: Option<i64>,

        /// Only refresh older than timestamp
        #[arg(short, long)]
        max_ts: Option<i64>,
    },
}

impl CacheCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> color_eyre::Result<()> {
        match &self.command {
            #[cfg(debug_assertions)]
            CacheSubcommands::CopyToDebug => {
                copy_to_debug();
            }

            CacheSubcommands::InitDatabase { reset } => {
                if *reset {
                    delete_database(&DB_LOCATION).expect("Failed to delete the database");
                }
                get_conn().await;
            }
            CacheSubcommands::LoadDump {
                username: _,
                path: _,
            } => {
                println!();
                println!("   {}", "Temporary unavailable :(".black().on_red().bold());
                println!("The data dumps currently don't have enough information to be useful, and may give false informations.");
                println!("So the feature is temporarily disabled");
                println!();
                println!("Please see: https://tickets.metabrainz.org/browse/LB-1687");
                //import_listen_dump(path, &Config::check_username(username)).await;
            }
            CacheSubcommands::Clear => {
                delete_database(&DB_LOCATION).expect("Failed to delete the database");
            }

            CacheSubcommands::ClearListens { user } => match user {
                Some(user) => {
                    sqlx::query!("DELETE FROM listens WHERE LOWER(user) = LOWER(?)", user)
                        .execute(conn)
                        .await
                        .expect("Couldn't delete listens");
                }
                None => {
                    sqlx::query!("DELETE FROM listens")
                        .execute(conn)
                        .await
                        .expect("Couldn't delete listens");
                }
            },

            CacheSubcommands::RefreshData {
                username,
                limit,
                max_ts,
            } => {
                refresh_data(
                    conn,
                    &Config::check_username(username),
                    limit.unwrap_or(i64::MAX),
                    max_ts.unwrap_or(i64::MAX),
                )
                .await;
            }
        }

        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum ClearTarget {
    All,
}

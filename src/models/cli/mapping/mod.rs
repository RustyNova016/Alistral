use clap::Parser;
use clap::Subcommand;
use tuillez::fatal_error::FatalError;

use crate::models::config::Config;
use crate::tools::listens::unlinked::unmapped_command;

use super::common::SortSorterBy;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct MappingCommand {
    #[command(subcommand)]
    subcommand: MappingSubcommands,
}

impl MappingCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        self.subcommand.run(conn).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MappingSubcommands {
    /// List all of your unlinked listens
    ///
    /// This command will list all your unmapped listens, grouped by similarity.
    /// It also gives a link to quickly look up the listen in listenbrainz, and go link it
    ///
    /// ```text
    ///
    /// (1) Paul's Dream (Dune) - Caster
    ///     -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>
    ///
    /// (7) Raise Your Weapon - KLOUD
    ///     -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>
    ///
    /// Total: 8 unlinked recordings
    ///
    /// ```
    ///
    /// > Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens
    /// > by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
    ///
    /// > This also means that the same recording can be shown twice in the list.
    /// > For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.
    ListUnmapped {
        /// Name of the user to fetch unlinked listen from
        username: Option<String>,

        /// Sort the listens by type
        #[arg(short, long)]
        sort: Option<SortSorterBy>,
    },
}

impl MappingSubcommands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::ListUnmapped { username, sort } => {
                unmapped_command(
                    conn,
                    &Config::check_username(username).to_lowercase(),
                    *sort,
                )
                .await;
            }
        }

        Ok(())
    }
}

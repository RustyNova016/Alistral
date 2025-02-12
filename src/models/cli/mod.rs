use std::io;

use cache::CacheCommand;
use clap::Command;
use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;
use clap_complete::Generator;
use clap_complete::Shell;
use clap_complete::generate;
use clap_verbosity_flag::InfoLevel;
use clap_verbosity_flag::Verbosity;
use config::ConfigCli;
use listens::ListenCommand;
use lookup::LookupCommand;
use mapping::MappingCommand;
use musicbrainz::MusicbrainzCommand;
use tuillez::fatal_error::FatalError;
use unstable::UnstableCommand;

use crate::models::cli::interzic::InterzicCommand;
use crate::models::cli::radio::RadioCommand;
use crate::tools::bumps::bump_command;
use crate::tools::bumps::bump_down_command;
use crate::tools::compatibility::compatibility_command;
use crate::tools::daily::daily_report;
use crate::tools::playlist::PlaylistCommand;
use crate::tools::stats::StatsCommand;

use super::config::Config;

pub mod cache;
pub mod common;
pub mod config;
pub mod interzic;
pub mod listens;
pub mod lookup;
pub mod mapping;
pub mod musicbrainz;
pub mod radio;
pub mod unstable;

/// Tools for Listenbrainz
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, hide = true)]
    pub markdown_help: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,

    #[arg(long, default_value_t = false)]
    no_cleanup: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<bool, FatalError> {
        // Invoked as: `$ my-app --markdown-help`
        if self.markdown_help {
            clap_markdown::print_help_markdown::<Self>();
            return Ok(false);
        }

        if let Some(generator) = self.generator {
            let mut cmd = Self::command();
            Self::print_completions(generator, &mut cmd);
            return Ok(false);
        }

        if let Some(command) = &self.command {
            command.run(conn).await?;
        }

        Ok(!self.no_cleanup)
    }

    fn print_completions<G: Generator>(gene: G, cmd: &mut Command) {
        generate(gene, cmd, cmd.get_name().to_string(), &mut io::stdout());
    }
}

/// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
///
/// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
///
/// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
#[derive(Parser, Debug, Clone)]
pub struct BumpCLI {
    /// The recording to bump
    pub recording: Option<String>,

    /// The duration the bump last for (Default: 3 months)
    #[arg(short, long)]
    pub duration: Option<String>,

    /// The multiplier added to the score (Default: 1.1)
    #[arg(short, long)]
    pub multiplier: Option<String>,

    #[arg(short, long)]
    pub username: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Bump(BumpCLI),

    /// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
    ///
    /// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
    ///    
    /// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
    BumpDown(BumpCLI),

    /// Commands to deal with the local cache
    Cache(CacheCommand),

    Compatibility {
        /// The name of the first user
        user_a: String,

        /// The name of the second user
        user_b: String,
    },

    /// Commands to deal with the app's configuration
    Config(ConfigCli),

    /// Daily report
    Daily {
        /// Name of the user to fetch stats listen from
        username: Option<String>,
    },

    /// Interact with the interzic database
    Interzic(InterzicCommand),

    /// Commands to edit listens
    Listens(ListenCommand),

    /// Get detailled information about an entity
    Lookup(LookupCommand),

    /// Commands for interacting with listen mappings
    Mapping(MappingCommand),

    /// Commands for musicbrainz stuff
    Musicbrainz(MusicbrainzCommand),

    /// Interact with playlists
    Playlist(PlaylistCommand),

    /// Generate radio playlists for you
    Radio(RadioCommand),

    /// Shows top statistics for a specific target
    ///
    /// Target is the entity type to group the stats by. Currently, those entities stats are implemented:
    ///
    /// - Recordings (`recording`)
    ///
    /// - Artists (`artist`)
    ///
    /// - Releases (`release`)
    ///
    /// - Release Groups (`release_group`)
    ///
    /// - Works (`work`)
    Stats(StatsCommand),

    Unstable(UnstableCommand),
}

impl Commands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::Stats(val) => val.run(conn).await?,

            Self::Compatibility { user_a, user_b } => {
                compatibility_command(conn, user_a, user_b).await;
            }

            Self::Radio(val) => val.run(conn).await?,

            Self::Cache(val) => val.run(conn).await?,

            Self::Config(val) => val.command.run().await?,

            Self::Daily { username } => daily_report(conn, &Config::check_username(username)).await,

            Self::Interzic(val) => val.run(conn).await?,

            Self::Listens(val) => val.run(conn).await?,

            Self::Lookup(val) => val.run(conn).await?,

            Self::Mapping(val) => val.run(conn).await?,

            Self::Musicbrainz(val) => val.run(conn).await,

            Self::Playlist(val) => val.run(conn).await?,

            Self::Bump(val) => bump_command(conn, val.clone()).await,

            Self::BumpDown(val) => bump_down_command(conn, val.clone()).await,

            Self::Unstable(val) => val.command.run(conn).await,
        }
        Ok(())
    }
}

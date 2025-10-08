use std::io;

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
use mapping::MappingCommand;
use tuillez::fatal_error::FatalError;
use unstable::UnstableCommand;

#[cfg(feature = "interzicf")]
use crate::models::cli::interzic::InterzicCommand;
#[cfg(feature = "radio")]
use crate::models::cli::radio::RadioCommand;
use crate::tools::bump::BumpCommand;
use crate::tools::bump::bump_down::BumpDownCommand;
use crate::tools::cache::CacheCommand;
use crate::tools::compatibility::compatibility_command;
use crate::tools::daily::DailyCommand;
#[cfg(feature = "lookup")]
use crate::tools::lookup::LookupCommand;
#[cfg(feature = "musicbrainz")]
use crate::tools::musicbrainz::MusicbrainzCommand;
#[cfg(feature = "interzicf")]
use crate::tools::playlist::PlaylistCommand;
#[cfg(feature = "stats")]
use crate::tools::stats::StatsCommand;

pub mod common;
pub mod config;
#[cfg(feature = "interzic")]
pub mod interzic;
pub mod listens;
pub mod mapping;
#[cfg(feature = "radio")]
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

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Bump(BumpCommand),

    /// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
    ///
    /// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
    ///    
    /// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
    BumpDown(BumpDownCommand),

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
    Daily(DailyCommand),

    #[cfg(feature = "interzic")]
    /// Interact with the interzic database
    Interzic(InterzicCommand),

    /// Commands to edit listens
    Listens(ListenCommand),

    #[cfg(feature = "lookup")]
    /// Get detailled information about an entity
    Lookup(LookupCommand),

    /// Commands for interacting with listen mappings
    Mapping(MappingCommand),

    #[cfg(feature = "musicbrainz")]
    /// Commands for musicbrainz stuff
    Musicbrainz(MusicbrainzCommand),

    #[cfg(feature = "interzicf")]
    /// Interact with playlists
    Playlist(PlaylistCommand),

    #[cfg(feature = "radio")]
    /// Generate radio playlists for you
    Radio(RadioCommand),

    #[cfg(feature = "stats")]
    /// Shows top statistics for a specific target
    Stats(StatsCommand),

    Unstable(UnstableCommand),
}

impl Commands {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError> {
        match self {
            Self::Bump(val) => val.run().await,
            Self::BumpDown(val) => val.run().await,
            Self::Cache(val) => val.run().await,

            #[cfg(feature = "stats")]
            Self::Stats(val) => val.run(conn).await?,

            Self::Compatibility { user_a, user_b } => {
                compatibility_command(conn, user_a, user_b).await;
            }

            #[cfg(feature = "radio")]
            Self::Radio(val) => val.run(conn).await?,

            Self::Config(val) => val.command.run().await?,

            Self::Daily(val) => val.run().await,

            #[cfg(feature = "interzic")]
            Self::Interzic(val) => val.run(conn).await?,

            Self::Listens(val) => val.run(conn).await?,

            #[cfg(feature = "lookup")]
            Self::Lookup(cmd) => cmd.run().await,

            Self::Mapping(val) => val.run(conn).await?,

            #[cfg(feature = "musicbrainz")]
            Self::Musicbrainz(val) => val.run(conn).await,

            #[cfg(feature = "interzicf")]
            Self::Playlist(val) => val.run(conn).await?,
            Self::Unstable(val) => val.command.run(conn).await,
        }

        Ok(())
    }
}

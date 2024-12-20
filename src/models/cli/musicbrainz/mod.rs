use clap::Parser;
use clap::Subcommand;

use crate::tools::musicbrainz::clippy::mb_clippy;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct MusicbrainzCommand {
    #[command(subcommand)]
    subcommand: MusicbrainzSubcommands,
}

impl MusicbrainzCommand {
    pub async fn run(&self) {
        self.subcommand.run().await;
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum MusicbrainzSubcommands {
    /// Search for potential mistakes, missing data and style issues. This allows to quickly pin down errors that can be corrected
    ///
    /// ⚠️ All tips are suggestions. Take them with a grain of salt. If you are unsure, it's preferable to skip.
    Clippy {
        /// The MBID of a recording to start from
        start_mbid: Option<String>,

        /// Whether to check FILO (first in, last out) instead of FIFO (first in, first out)
        #[arg(short, long)]
        new_first: bool,

        /// List of lints that should only be checked (Note: Put this argument last or before another argument)
        #[arg(short, long, num_args = 0..)]
        whitelist: Option<Vec<String>>,

        /// List of lints that should not be checked (Note: Put this argument last or before another argument)
        #[arg(short, long, num_args = 0..)]
        blacklist: Option<Vec<String>>,
    },
}

impl MusicbrainzSubcommands {
    pub async fn run(&self) {
        match self {
            Self::Clippy {
                start_mbid,
                new_first,
                whitelist,
                blacklist,
            } => {
                let mbid = start_mbid
                    .clone()
                    .unwrap_or_else(|| "8f3471b5-7e6a-48da-86a9-c1c07a0f47ae".to_string());

                let filter = if let Some(whitelist) = whitelist {
                    WhitelistBlacklist::WhiteList(whitelist.clone())
                } else if let Some(blacklist) = blacklist {
                    WhitelistBlacklist::BlackList(blacklist.clone())
                } else {
                    WhitelistBlacklist::BlackList(Vec::new())
                };

                mb_clippy(
                    &read_mbid_from_input(&mbid).expect("Couldn't read mbid"),
                    *new_first,
                    &filter,
                )
                .await;
            }
        }
    }
}

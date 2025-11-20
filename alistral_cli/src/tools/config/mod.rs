use std::fs::remove_file;
use std::io;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;

use crate::tools::cache::clear::CacheClearCommand;
use crate::tools::cache::copy_to_debug::CacheCopyToDebugCommand;



pub mod listenbrainz_domain;
/// Commands that interface with the cache of the app
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: ConfigSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigSubcommands {
    #[clap(aliases = &["listenbrainzz_domain"])]
    ListenbrainzDomain
}

impl ConfigCommand {
    pub async fn run(&self) {
        match &self.command {
            ConfigSubcommands::ListenbrainzDomain(val) => val.run(),
        }
    }
}


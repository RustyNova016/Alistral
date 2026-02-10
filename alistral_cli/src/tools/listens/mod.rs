use clap::Parser;
use clap::Subcommand;

use crate::tools::listens::clear::ListensClearCommand;
use crate::tools::listens::import::ListenImportDumpCommand;
use crate::tools::listens::reload::ListenReloadCommand;
use crate::tools::listens::remap_msid::ListenRemapMsidCommand;
use crate::tools::listens::submit::ListenSubmitCommand;
use crate::tools::listens::unlinked::ListenUnlinkedCommand;
use crate::tools::listens::wrong_mapping::ListenWrongMappingCommand;

pub mod clear;
pub mod import;
pub mod reload;
pub mod remap_msid;
pub mod submit;
pub mod unlinked;
pub mod wrong_mapping;

/// Commands to interface with listens
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ListenCommand {
    #[command(subcommand)]
    subcommand: ListenSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListenSubcommands {
    Clear(ListensClearCommand),
    ImportDump(ListenImportDumpCommand),
    Reload(ListenReloadCommand),
    RemapMsid(ListenRemapMsidCommand),
    Submit(ListenSubmitCommand),
    #[clap(aliases = &["unmapped"])]
    Unlinked(ListenUnlinkedCommand),
    WrongMapping(ListenWrongMappingCommand),
}

impl ListenCommand {
    pub async fn run(&self) {
        match &self.subcommand {
            ListenSubcommands::Clear(cmd) => cmd.run().await,
            ListenSubcommands::ImportDump(cmd) => cmd.run().await,
            ListenSubcommands::Reload(cmd) => cmd.run().await,
            ListenSubcommands::RemapMsid(cmd) => cmd.run().await,
            ListenSubcommands::Submit(cmd) => cmd.run().await,
            ListenSubcommands::Unlinked(cmd) => cmd.run().await,
            ListenSubcommands::WrongMapping(cmd) => cmd.run().await,
        }
    }
}

use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

use crate::tools::interzic::get_mapping::GetMappingCommand;
use crate::tools::interzic::overwrite::OverwriteCommand;
use crate::tools::interzic::reload::ReloadCommand;
use crate::tools::interzic::reverse_mapping::ReverseMappingCommand;

pub mod get_mapping;
pub mod overwrite;
pub mod reload;
pub mod reverse_mapping;

/// Interact with the interzic database
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct InterzicCommand {
    #[command(subcommand)]
    pub command: InterzicSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InterzicSubcommands {
    GetMapping(GetMappingCommand),
    Reload(ReloadCommand),
    ReverseMapping(ReverseMappingCommand),
    Overwrite(OverwriteCommand),
}

impl InterzicCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        match &self.command {
            InterzicSubcommands::GetMapping(args) => Ok(args.run().await?),
            InterzicSubcommands::Reload(args) => Ok(args.run().await?),
            InterzicSubcommands::ReverseMapping(args) => Ok(args.run().await?),
            InterzicSubcommands::Overwrite(args) => Ok(args.run().await?),
        }
    }
}

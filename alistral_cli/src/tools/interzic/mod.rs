use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

#[cfg(any(feature = "youtube"))]
use crate::tools::interzic::get_mapping::GetMappingCommand;
#[cfg(any(feature = "youtube"))]
use crate::tools::interzic::overwrite::OverwriteCommand;
use crate::tools::interzic::reload::ReloadCommand;
#[cfg(any(feature = "youtube"))]
use crate::tools::interzic::reverse_mapping::ReverseMappingCommand;

#[cfg(any(feature = "youtube"))]
pub mod get_mapping;
#[cfg(any(feature = "youtube"))]
pub mod overwrite;
pub mod reload;
#[cfg(any(feature = "youtube"))]
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
    #[cfg(any(feature = "youtube"))]
    GetMapping(GetMappingCommand),
    Reload(ReloadCommand),
    #[cfg(any(feature = "youtube"))]
    ReverseMapping(ReverseMappingCommand),
    #[cfg(any(feature = "youtube"))]
    Overwrite(OverwriteCommand),
}

impl InterzicCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        match &self.command {
            #[cfg(any(feature = "youtube"))]
            InterzicSubcommands::GetMapping(args) => Ok(args.run().await?),
            InterzicSubcommands::Reload(args) => Ok(args.run().await?),
            #[cfg(any(feature = "youtube"))]
            InterzicSubcommands::ReverseMapping(args) => Ok(args.run().await?),
            #[cfg(any(feature = "youtube"))]
            InterzicSubcommands::Overwrite(args) => Ok(args.run().await?),
        }
    }
}

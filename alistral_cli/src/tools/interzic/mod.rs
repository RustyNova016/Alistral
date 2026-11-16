use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

#[cfg(feature = "subsonic")]
use crate::tools::interzic::add_subsonic::InterzicAddSubsonicCommand;
#[cfg(feature = "subsonic")]
use crate::tools::interzic::get_mapping::GetMappingCommand;
#[cfg(feature = "subsonic")]
use crate::tools::interzic::overwrite::OverwriteCommand;
#[cfg(feature = "subsonic")]
use crate::tools::interzic::reverse_mapping::ReverseMappingCommand;

#[cfg(feature = "subsonic")]
pub mod add_subsonic;
#[cfg(feature = "subsonic")]
pub mod get_mapping;
#[cfg(feature = "subsonic")]
pub mod overwrite;
#[cfg(feature = "subsonic")]
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
    #[cfg(feature = "subsonic")]
    AddSubsonic(InterzicAddSubsonicCommand),
    #[cfg(feature = "subsonic")]
    GetMapping(GetMappingCommand),
    #[cfg(feature = "subsonic")]
    ReverseMapping(ReverseMappingCommand),
    #[cfg(feature = "subsonic")]
    Overwrite(OverwriteCommand),
}

impl InterzicCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        match &self.command {
            #[cfg(feature = "subsonic")]
            InterzicSubcommands::AddSubsonic(args) => Ok(args.run().await?),
            #[cfg(feature = "subsonic")]
            InterzicSubcommands::GetMapping(args) => Ok(args.run().await?),
            #[cfg(feature = "subsonic")]
            InterzicSubcommands::ReverseMapping(args) => Ok(args.run().await?),
            #[cfg(feature = "subsonic")]
            InterzicSubcommands::Overwrite(args) => Ok(args.run().await?),
        }
    }
}

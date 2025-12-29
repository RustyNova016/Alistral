use clap::{Parser, Subcommand};
use tuillez::fatal_error::FatalError;

#[cfg(feature = "subsonic")]
use crate::tools::interzic::add_subsonic::InterzicAddSubsonicCommand;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
use crate::tools::interzic::get_mapping::GetMappingCommand;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
use crate::tools::interzic::overwrite::OverwriteCommand;
use crate::tools::interzic::reload::ReloadCommand;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
use crate::tools::interzic::reverse_mapping::ReverseMappingCommand;

#[cfg(any(feature = "youtube", feature = "subsonic"))]
pub mod add_subsonic;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
pub mod get_mapping;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
pub mod overwrite;
pub mod reload;
#[cfg(any(feature = "youtube", feature = "subsonic"))]
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
    #[cfg(any(feature = "youtube", feature = "subsonic"))]
    GetMapping(GetMappingCommand),
    Reload(ReloadCommand),
    #[cfg(any(feature = "youtube", feature = "subsonic"))]
    ReverseMapping(ReverseMappingCommand),
    #[cfg(any(feature = "youtube", feature = "subsonic"))]
    Overwrite(OverwriteCommand),
}

impl InterzicCommand {
    pub async fn run(&self) -> Result<(), FatalError> {
        match &self.command {
            #[cfg(feature = "subsonic")]
            InterzicSubcommands::AddSubsonic(args) => Ok(args.run().await?),
            #[cfg(any(feature = "youtube", feature = "subsonic"))]
            InterzicSubcommands::GetMapping(args) => Ok(args.run().await?),
            InterzicSubcommands::Reload(args) => Ok(args.run().await?),
            #[cfg(any(feature = "youtube", feature = "subsonic"))]
            InterzicSubcommands::ReverseMapping(args) => Ok(args.run().await?),
            #[cfg(any(feature = "youtube", feature = "subsonic"))]
            InterzicSubcommands::Overwrite(args) => Ok(args.run().await?),
        }
    }
}

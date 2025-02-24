use crate::models::cli::common::ConfigBool;
use crate::models::config::global_config::CONFIG;
use clap::Parser;
use clap::Subcommand;
use clap::command;
use tracing::info;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ListenConfigCli {
    #[command(subcommand)]
    subcommand: ListenConfigSubcommands,
}

impl ListenConfigCli {
    pub async fn run(&self) -> Result<(), crate::Error> {
        self.subcommand.run().await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListenConfigSubcommands {
    /// Toggle / Set whether the unmapped listens should be automatically updated when fetching listens
    RefreshUnmappedListens {
        /// What do you want it set to?
        state: ConfigBool,
    },
}

impl ListenConfigSubcommands {
    pub async fn run(&self) -> Result<(), crate::Error> {
        match self {
            Self::RefreshUnmappedListens { state } => {
                let mut config_lock = CONFIG.write().await;

                config_lock.listens.config_refresh_unmapped_listens(*state);

                info!(
                    "Successfully set `RefreshUnmappedListens` to {}",
                    config_lock.listens.refresh_unmapped_listens
                );
            }
        }

        Ok(())
    }
}

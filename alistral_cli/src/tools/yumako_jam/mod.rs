use crate::tools::yumako_jam::run::YumakoRunCommand;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;

pub mod run;

#[derive(clap::Parser, Debug, Clone)]
pub struct YumakoCommand {
    #[command(subcommand)]
    pub command: YumakoSubcommands,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum YumakoSubcommands {
    Run(YumakoRunCommand),
}

impl YumakoCommand {
    pub async fn run(&self) -> Result<(), YumakoRunCommandError> {
        match &self.command {
            YumakoSubcommands::Run(val) => val.run().await,
        }
    }
}

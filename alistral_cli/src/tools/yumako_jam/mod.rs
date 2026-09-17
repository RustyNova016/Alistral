use crate::tools::yumako_jam::info::YumakoInfoCommand;
use crate::tools::yumako_jam::run::YumakoRunCommand;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;

pub mod run;
pub mod info;

#[derive(clap::Parser, Debug, Clone)]
pub struct YumakoCommand {
    #[command(subcommand)]
    pub command: YumakoSubcommands,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum YumakoSubcommands {
    Info(YumakoInfoCommand),
    Run(YumakoRunCommand),
}

impl YumakoCommand {
    pub async fn run(&self) -> Result<(), YumakoRunCommandError> {
        match &self.command {
            YumakoSubcommands::Info(val) => Ok(val.run().unwrap()),
            YumakoSubcommands::Run(val) => val.run().await,
        }
    }
}

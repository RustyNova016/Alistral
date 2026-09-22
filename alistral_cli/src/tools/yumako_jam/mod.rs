use snafu::ResultExt as _;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::yumako_jam::info::YumakoInfoCommand;
use crate::tools::yumako_jam::run::YumakoRunCommand;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;
use crate::utils::yumako_jams::get_radio::YumakoGetRadioError;

pub mod info;
pub mod run;

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
    pub async fn run(&self) -> Result<(), YumakoCommandError> {
        match &self.command {
            YumakoSubcommands::Info(val) => val.run().context(YumakoInfoCommandSnafu)?,
            YumakoSubcommands::Run(val) => val.run().await.context(YumakoRunCommandSnafu)?,
        };

        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
#[expect(
    clippy::large_enum_variant,
    reason = "Snafu doesn't support boxing easily"
)]
pub enum YumakoCommandError {
    YumakoRunCommandError {
        source: YumakoRunCommandError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
    YumakoInfoCommandError {
        source: YumakoGetRadioError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for YumakoCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::YumakoRunCommandError { source, .. } => source.get_friendly_error(),
            Self::YumakoInfoCommandError { source, .. } => source.get_friendly_error(),
        }
    }
}

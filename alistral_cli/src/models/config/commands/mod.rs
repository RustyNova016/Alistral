#[cfg(feature = "report")]
pub mod reports;
use crate::models::config::commands::daily::DailyCommandConfig;
#[cfg(feature = "report")]
use crate::models::config::commands::reports::ReportConfig;

pub mod daily;

/// Configuration for the commands
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct CommandConfig {
    daily: Option<DailyCommandConfig>,
    #[cfg(feature = "report")]
    reports: Option<ReportConfig>,
}

impl CommandConfig {
    pub fn daily(&self) -> Option<&DailyCommandConfig> {
        self.daily.as_ref()
    }

    #[cfg(feature = "report")]
    pub fn reports(&self) -> Option<&ReportConfig> {
        self.reports.as_ref()
    }
}

use crate::models::config::commands::daily::DailyCommandConfig;

pub mod daily;

/// Configuration for the commands
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct CommandConfig {
    daily: Option<DailyCommandConfig>,
}

impl CommandConfig {
    pub fn daily(&self) -> Option<&DailyCommandConfig> {
        self.daily.as_ref()
    }
}

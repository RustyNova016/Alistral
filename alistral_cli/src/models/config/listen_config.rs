use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::cli::common::ConfigBool;

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ListenConfig {
    pub refresh_unmapped_listens: bool,
}

impl ListenConfig {
    pub fn config_refresh_unmapped_listens(&mut self, state: ConfigBool) {
        self.refresh_unmapped_listens = match state {
            ConfigBool::False => false,
            ConfigBool::True => true,
            ConfigBool::Toggle => !self.refresh_unmapped_listens,
        }
    }
}

impl Default for ListenConfig {
    fn default() -> Self {
        Self {
            refresh_unmapped_listens: true,
        }
    }
}

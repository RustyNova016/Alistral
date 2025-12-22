use serde::Deserialize;
use serde::Serialize;

use crate::models::config::interzic::subsonic_config::SubsonicConfig;

pub mod subsonic_config;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct InterzicConfig {
    pub subsonic_clients: Vec<SubsonicConfig>,
}

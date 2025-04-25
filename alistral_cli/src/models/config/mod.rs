pub mod whitelisted_wrong_mappings;
use bumps::BumpList;
use clap::CommandFactory;
use config_guard::ConfigGuard;
use config_trait::ConfigFile;
use derive_getters::Getters;
use listen_config::ListenConfig;
use mapper::MapperConfig;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use super::cli::Cli;

pub mod bumps;
pub mod config_guard;
pub mod config_trait;
pub mod global_config;
pub mod listen_config;
pub mod mapper;
pub mod recording_timeout;

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    /// Saved usertokens
    tokens: HashMap<String, String>,

    pub(super) mapper: Option<MapperConfig>,

    #[serde(default)]
    pub listens: ListenConfig,

    pub default_user: Option<String>,

    #[serde(default)]
    pub bumps: BumpList,

    pub artist_listened_to: Option<Decimal>,

    #[serde(default = "default_lb_url")]
    pub listenbrainz_url: String,

    #[serde(default = "default_mb_url")]
    pub musicbrainz_url: String,
}

impl Config {
    pub fn load_or_panic() -> ConfigGuard<Self> {
        Self::load().expect("Cannot load the configuration file. Aborting")
    }

    // Token methods

    pub fn set_token(&mut self, username: String, token: String) {
        self.tokens.insert(username.to_lowercase(), token);
    }

    pub fn check_token(username: &str, arg: &Option<String>) -> String {
        if let Some(arg) = arg {
            return arg.clone();
        }

        match Self::load_or_panic().read_or_panic().tokens.get(&username.to_lowercase()) {
            Some(val) => val.clone(),
            None => {
                Cli::command()
                    .error(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        format!("No token was provided, and no tokens have been associated to {username}. Try adding your token to the command, or set the user's token with `config set-token <USERNAME> <TOKEN>`"),
                    )
                    .exit()
            }
        }
    }

    // Username methods
    pub fn get_default_user() -> Option<String> {
        Self::load_or_panic().read_or_panic().default_user.clone()
    }

    pub fn check_username(s: &Option<String>) -> String {
        if let Some(s) = s {
            return s.clone();
        }

        match Self::get_default_user() {
            Some(val) => val,
            None => {
                Cli::command()
                    .error(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "No username was provided, and the default username isn't set. Try adding your username to the command, or set the default username with `config default-user <USERNAME>`",
                    )
                    .exit()
            }
        }
    }

    pub fn get_username_or_panic(&self, user: Option<String>) -> String {
        match user.or(self.default_user.clone()) {
            Some(v) => v,
            None => {
                Cli::command()
                    .error(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "No username was provided, and the default username isn't set. Try adding your username to the command, or set the default username with `config default-user <USERNAME>`",
                    )
                    .exit()
            }
        }
    }

    pub fn get_artist_listened_to_threshold(&self) -> Decimal {
        self.artist_listened_to
            .unwrap_or_else(|| Decimal::new(2, 0))
    }
}

impl ConfigFile for Config {
    fn file_name() -> &'static str {
        "config.json"
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            artist_listened_to: Some(Decimal::new(2, 0)),
            tokens: Default::default(),
            mapper: Default::default(),
            listens: Default::default(),
            default_user: Default::default(),
            bumps: Default::default(),
            listenbrainz_url: default_lb_url(),
            musicbrainz_url: default_mb_url(),
        }
    }
}

fn default_lb_url() -> String {
    "https://api.listenbrainz.org/1/".to_string()
}

fn default_mb_url() -> String {
    "http://musicbrainz.org/ws/2".to_string()
}

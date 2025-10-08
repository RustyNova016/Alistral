use std::path::PathBuf;

use crate::models::config::Config;

pub mod get_entities;
pub mod mbids;

pub struct UserInputParser;

impl UserInputParser {
    /// Return the provided username or the default username
    pub fn username_or_default(username: &Option<String>) -> String {
        Config::check_username(username)
    }

    /// Return the provided token or the default user token
    pub fn user_token_or_default(username: &Option<String>, token: &Option<String>) -> String {
        Config::check_token(&Self::username_or_default(username), token)
    }

    pub fn parse_file_path(path: &str) -> PathBuf {
        match PathBuf::from(path).canonicalize() {
            Ok(path) => path,
            Err(_) => {
                tracing::error!("Couldn't find file `{path}`");
                std::process::exit(2);
            }
        }
    }
}

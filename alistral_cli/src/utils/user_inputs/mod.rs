use crate::models::config::Config;

pub mod get_entities;
pub mod mbids;

pub struct UserInputParser;

impl UserInputParser {
    /// Return the provided username or the default username
    pub fn username_or_default(username: &Option<String>) -> String {
        Config::check_username(username)
    }
}

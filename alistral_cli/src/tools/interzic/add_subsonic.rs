use clap::Parser;

use crate::models::config::global_config::CONFIG;
use crate::models::config::interzic::subsonic_config::SubsonicConfig;

/// Add a subsonic client
#[derive(Parser, Debug, Clone)]
pub struct InterzicAddSubsonicCommand {
    /// The name of this instance
    name: String,

    /// The url of the instance
    url: String,

    /// The username of the instance user's
    username: String,

    /// The password of the user
    password: String,
}

impl InterzicAddSubsonicCommand {
    pub async fn run(&self) -> Result<(), crate::Error> {
        let instance = SubsonicConfig {
            name: self.name.to_lowercase(),
            password: self.password.to_string(),
            url: self.url.to_string(),
            username: self.username.to_string(),
        };

        CONFIG
            .write()
            .await
            .interzic
            .subsonic_clients
            .push(instance);

        Ok(())
    }
}

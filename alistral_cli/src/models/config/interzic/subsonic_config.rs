use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubsonicConfig {
    /// The name of the subsonic instance
    pub name: String,

    pub url: String,

    /// The username of the user
    pub username: String,
    /// The password of the user
    pub password: String
}
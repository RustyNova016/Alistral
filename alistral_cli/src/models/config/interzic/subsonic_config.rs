use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct SubsonicConfig {
    /// The name of the subsonic instance
    pub name: String,

    /// The url of the subsonic instance
    pub url: String,

    /// The username of the user
    pub username: String,
    /// The password of the user
    pub password: String,
}

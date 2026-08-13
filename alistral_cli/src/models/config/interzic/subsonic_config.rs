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

    /// If set to true, only subsonic tracks that match the MBID of the requested recording are mapped
    ///
    /// Only set this if your subsonic server is capable of searching by MBID (Like navidrome),
    /// and your collection has been passed through Musicbrainz Picard
    pub strict_mbid: Option<bool>,
}

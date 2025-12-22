pub mod cache;
pub mod error;
pub mod playlists;
pub mod search;

#[derive(Debug, bon::Builder)]
pub struct SubsonicClient {
    /// Name of the subsonic instance. This isn't the name of the underlying application, but the identifier for this instance
    name: String,

    inner_client: submarine::Client,
}

impl SubsonicClient {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn service_name(&self) -> String {
        format!("subsonic-{}", self.name)
    }
}

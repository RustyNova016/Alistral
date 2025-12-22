use submarine::auth::AuthBuilder;

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
    pub fn new(name: String, url: &str, username: &str, password: &str) -> Self {
        let auth = AuthBuilder::new(username, "v1.16.1")
            .client_name("Interzic") //TODO: Set from client
            .hashed(password);

        let sub_client = submarine::Client::new(url, auth);

        Self {
            name,
            inner_client: sub_client,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn service_name(&self) -> String {
        format!("subsonic-{}", self.name)
    }
}

use crate::InterzicClient;
use crate::models::services::subsonic::SubsonicClient;

impl InterzicClient {
    pub fn add_subsonic_client(&mut self, client: SubsonicClient) {
        self.subsonic_clients
            .insert(client.name().to_string(), client);
    }

    pub fn get_subsonic_client(&self, name: &str) -> Option<&SubsonicClient> {
        self.subsonic_clients.get(name)
    }
}

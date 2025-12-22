use snafu::ResultExt;
use tracing::warn;

use crate::InterzicClient;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::subsonic::SubsonicClient;
use crate::models::services::subsonic::error::SubsonicSnafu;

impl SubsonicClient {
    pub async fn create_playlist(
        &self,
        client: &InterzicClient,
        playlist: PlaylistStub,
        user_overwrite: Option<String>,
    ) -> Result<submarine::data::PlaylistWithSongs, super::error::SubsonicServiceError> {
        let mut items = Vec::new();

        for recording in playlist.recordings {
            let rec = self
                .get_of_fetch(client, &recording, user_overwrite.clone())
                .await
                .unwrap();

            if let Some(rec) = rec {
                items.push(rec);
            }

            warn!(
                "Recording `{}` not found in the subsonic server `{}`",
                recording, self.name
            );
        }

        self.inner_client
            .create_playlist(playlist.title, items)
            .await
            .context(SubsonicSnafu)
    }
}

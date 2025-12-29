use snafu::ResultExt;
use tracing::instrument;
use tracing::warn;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::InterzicClient;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::subsonic::SubsonicClient;
use crate::models::services::subsonic::error::SubsonicSnafu;

impl SubsonicClient {
    #[instrument(skip(client, playlist), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn create_playlist(
        &self,
        client: &InterzicClient,
        playlist: PlaylistStub,
        user_overwrite: Option<String>,
    ) -> Result<submarine::data::PlaylistWithSongs, super::error::SubsonicServiceError> {
        let mut items = Vec::new();
        pg_counted!(playlist.recordings.len(), "Creating playlist");

        for recording in playlist.recordings {
            let rec = self
                .get_or_fetch(client, &recording, user_overwrite.clone())
                .await
                .unwrap();

            if let Some(rec) = rec {
                items.push(rec);
            } else {
                warn!(
                    "Recording `{}` not found in the subsonic server `{}`",
                    format!("{} {}", recording.title, recording.artist_credits),
                    self.name
                );
            }

            pg_inc!();
        }

        self.inner_client
            .create_playlist(playlist.title, items)
            .await
            .context(SubsonicSnafu)
    }
}

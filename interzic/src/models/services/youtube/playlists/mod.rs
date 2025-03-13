use core::num::NonZeroU32;

use google_youtube3::api::Playlist;
use google_youtube3::api::PlaylistSnippet;
use governor::clock::QuantaClock;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use governor::state::InMemoryState;
use governor::state::NotKeyed;
use governor::Quota;
use governor::RateLimiter;
use tracing::debug;
use tracing::info;

use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::youtube::error::InterzicYoutubeError;
use crate::models::services::youtube::error::YoutubeError;
use crate::models::services::youtube::Youtube;
use crate::InterzicClient;

pub mod add_playlist_item;

pub type YoutubeRateLimit =
    RateLimiter<NotKeyed, InMemoryState, QuantaClock, NoOpMiddleware<QuantaInstant>>;

impl Youtube {
    pub async fn create_playlist(
        client: &InterzicClient,
        playlist: PlaylistStub,
        user_overwrite: Option<String>,
    ) -> Result<String, crate::Error> {
        info!("Creating playlist");

        let rate_limit = RateLimiter::direct(Quota::per_second(NonZeroU32::new(1).unwrap()));
        let response = client
            .youtube_client()?
            .playlists()
            .insert(playlist.clone().to_yt_playlist())
            .add_part("id")
            .add_part("snippet")
            .doit()
            .await
            .map_err(YoutubeError::from)
            .map_err(InterzicYoutubeError::PlaylistCreateError)?;

        let playlist_id = response
            .1
            .id
            .ok_or(YoutubeError::MissingPlaylistIDError)
            .map_err(InterzicYoutubeError::PlaylistCreateError)?;

        debug!("Adding recordings to playlist");
        Self::add_recordings_to_playlist(
            client,
            &playlist_id,
            playlist,
            rate_limit,
            user_overwrite,
        )
        .await?;

        Ok(playlist_id)
    }
}

impl PlaylistStub {
    pub fn to_yt_playlist(self) -> Playlist {
        Playlist {
            snippet: Some(PlaylistSnippet {
                title: Some(self.title),
                description: Some(self.description),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

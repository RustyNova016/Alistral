use core::num::NonZeroU32;

use google_youtube3::api::Playlist;
use google_youtube3::api::PlaylistItem;
use google_youtube3::api::PlaylistItemSnippet;
use google_youtube3::api::PlaylistSnippet;
use google_youtube3::api::ResourceId;
use governor::clock::QuantaClock;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use governor::state::InMemoryState;
use governor::state::NotKeyed;
use governor::Quota;
use governor::RateLimiter;
use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::warn;
use tracing::Span; 
use tracing_indicatif::span_ext::IndicatifSpanExt as _;

use crate::models::playlist_stub;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::youtube::error::BadRequestError;
use crate::models::services::youtube::error::YoutubeError;
use crate::models::services::youtube::Youtube;
use crate::InterzicClient;

impl Youtube {
    pub async fn create_playlist(
        client: &InterzicClient,
        playlist: PlaylistStub,
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
            .map_err(YoutubeError::PlaylistCreateError)?;

        let playlist_id = response.1.id.expect("No id returned"); //TODO: Proper error

        for recording in playlist.recordings {
            //TODO: Check number of recordings missing
            //TODO: User overwrite
            let Some(video_id) = Self::get_or_query(client, recording, None).await? else {
                continue;
            };
            rate_limit.until_ready().await;

            Self::add_video_to_playlist(client, playlist_id.clone(), video_id).await?;
            Span::current().pb_inc(1);
        }

        Ok(playlist_id)
    }

    #[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn add_recordings_to_playlist(
        client: &InterzicClient,
        playlist_id: String,
        playlist: PlaylistStub,
        rate_limit: RateLimiter<
            NotKeyed,
            InMemoryState,
            QuantaClock,
            NoOpMiddleware<QuantaInstant>,
        >,
    ) -> Result<(), crate::Error> {
        //TODO: use pg_counted from alistral_core > Move CLI stuff to separate crate
        Span::current().pb_set_length(playlist.recordings.len() as u64);
        Span::current().pb_set_message("Creating playlist");

        for recording in playlist.recordings {
            //TODO: Check number of recordings missing
            //TODO: User overwrite
            let Some(video_id) = Self::get_or_query(client, recording, None).await? else {
                continue;
            };
            rate_limit.until_ready().await;

            Self::add_video_to_playlist(client, playlist_id.clone(), video_id).await?;
            Span::current().pb_inc(1);
        }

        Ok(())
    }

    async fn add_video_to_playlist(
        client: &InterzicClient,
        playlist_id: String,
        video_id: String,
    ) -> Result<(), YoutubeError> {
        for i in 0..5 {
            let responce = client
                .youtube_client()?
                .playlist_items()
                .insert(recording_to_playlist_item(
                    playlist_id.clone(),
                    video_id.clone(),
                ))
                .add_part("id")
                .add_part("snippet")
                .doit()
                .await;

            let Err(err) = responce else {
                return Ok(());
            };

            if i != 5 && !is_retry_error(&err) {
                return Err(YoutubeError::PlaylistInsertError(err));
            }

            warn!(
                "Error while sending video {}. Retrying ({}/5)",
                video_id,
                i + 1
            )
        }

        error!("Couldn't send video {}. Skipping to next track", video_id);
        Ok(())
    }
}

fn is_retry_error(err: &google_youtube3::common::Error) -> bool {
    let google_youtube3::common::Error::BadRequest(err) = err else {
        return false;
    };

    let err: Result<BadRequestError, serde_json::Error> = serde_json::from_value(err.clone());

    let Ok(err) = err else {
        return false;
    };

    err.error.code == 409
        && err
            .error
            .errors
            .iter()
            .any(|err| err.reason == "SERVICE_UNAVAILABLE")
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

fn recording_to_playlist_item(playlist_id: String, vid_id: String) -> PlaylistItem {
    PlaylistItem {
        snippet: Some(PlaylistItemSnippet {
            playlist_id: Some(playlist_id),
            resource_id: Some(ResourceId {
                kind: Some("youtube#video".to_string()),
                video_id: Some(vid_id),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

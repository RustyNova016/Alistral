use core::num::NonZeroU32;

use google_youtube3::api::Playlist;
use google_youtube3::api::PlaylistItem;
use google_youtube3::api::PlaylistItemSnippet;
use google_youtube3::api::PlaylistSnippet;
use google_youtube3::api::ResourceId;
use governor::Quota;
use governor::RateLimiter;

use crate::database::recording;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::youtube::error::BadRequestError;
use crate::models::services::youtube::error::BadRequestErrorErrorItem;
use crate::models::services::youtube::error::YoutubeError;
use crate::models::services::youtube::Youtube;
use crate::InterzicClient;

impl Youtube {
    pub async fn create_playlist(
        client: &InterzicClient,
        playlist: PlaylistStub,
    ) -> Result<String, crate::Error> {
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
        }

        Ok(playlist_id)
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

            println!("Error while sending video. Retrying ({}/5)", i + 1)
        }
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
        let mut playlist = Playlist::default();
        let mut playlist_snippet = PlaylistSnippet::default();

        //TODO: Visibility

        playlist_snippet.title = Some(self.title);
        playlist_snippet.description = Some(self.description);

        playlist.snippet = Some(playlist_snippet);

        playlist
    }
}

fn recording_to_playlist_item(playlist_id: String, vid_id: String) -> PlaylistItem {
    let mut playlist = PlaylistItem::default();
    let mut playlist_snippet = PlaylistItemSnippet::default();
    let mut ressource_id = ResourceId::default();

    ressource_id.kind = Some("youtube#video".to_string());
    ressource_id.video_id = Some(vid_id);

    playlist_snippet.playlist_id = Some(playlist_id);
    playlist_snippet.resource_id = Some(ressource_id);

    playlist.snippet = Some(playlist_snippet);

    playlist
}

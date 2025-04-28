use google_youtube3::api::PlaylistItem;
use google_youtube3::api::PlaylistItemSnippet;
use google_youtube3::api::ResourceId;
use tracing::debug;
use tracing::error;
use tracing::instrument;
use tracing::warn;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::InterzicClient;
use crate::client::YoutubeClient;
use crate::models::messy_recording::MessyRecording;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::youtube::Youtube;
use crate::models::services::youtube::error::InterzicYoutubeError;
use crate::models::services::youtube::error::YoutubeError;
use crate::models::services::youtube::error::is_add_404_video_error;
use crate::models::services::youtube::playlists::YoutubeRateLimit;
use crate::try_err;

impl Youtube {
    #[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn add_recordings_to_playlist(
        client: &InterzicClient,
        playlist_id: &String,
        playlist: PlaylistStub,
        rate_limit: YoutubeRateLimit,
        user_overwrite: Option<String>,
    ) -> Result<(), crate::Error> {
        pg_counted!(playlist.recordings.len(), "Creating playlist");
        let mut errors = Vec::new();

        for recording in playlist.recordings {
            match Self::add_recording_to_playlist(
                client,
                playlist_id.clone(),
                &recording,
                &rate_limit,
                user_overwrite.clone(),
            )
            .await
            {
                Ok(_) => {}
                Err(err) => {
                    error!(
                        "Error while sending playlist item: \n{}. \n\nRecording: `{}` \n\nSkipping...",
                        err.to_string(),
                        recording,
                    );
                    errors.push(err)
                }
            }

            pg_inc!();
        }

        Ok(())
    }

    async fn add_recording_to_playlist(
        client: &InterzicClient,
        playlist_id: String,
        recording: &MessyRecording,
        rate_limit: &YoutubeRateLimit,
        user_overwrite: Option<String>,
    ) -> Result<(), crate::Error> {
        debug!("Sending recording: {recording}");
        let video_id = Youtube::get_id_or_query(client, recording, user_overwrite)
            .await?
            .ok_or_else(|| InterzicYoutubeError::RecordingSearchNotFoundError(recording.clone()))?;

        add_video_to_playlist_with_retries(
            client.youtube_client()?,
            playlist_id,
            video_id,
            rate_limit,
        )
        .await
        .map_err(InterzicYoutubeError::from)?;

        Ok(())
    }
}

async fn add_video_to_playlist_with_retries(
    client: &YoutubeClient,
    playlist_id: String,
    video_id: String,
    rate_limit: &YoutubeRateLimit,
) -> Result<(), YoutubeError> {
    for i in 0..5 {
        rate_limit.until_ready().await;
        let err =
            try_err!(add_video_to_playlist(client, playlist_id.clone(), video_id.clone()).await);

        if i == 5 || !err.is_bad_service_error() {
            return Err(err);
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

async fn add_video_to_playlist(
    client: &YoutubeClient,
    playlist_id: String,
    video_id: String,
) -> Result<(), YoutubeError> {
    let err = try_err!(add_item_request(client, playlist_id, video_id.clone()).await);

    if is_add_404_video_error(&err) {
        Err(YoutubeError::Add404VideoError(err, video_id))
    } else {
        Err(YoutubeError::from(err))
    }
}

/// Send the "add item" request
async fn add_item_request(
    client: &YoutubeClient,
    playlist_id: String,
    video_id: String,
) -> Result<(), google_youtube3::common::Error> {
    debug!("Sending video `{video_id}` to playlist {playlist_id}");
    client
        .playlist_items()
        .insert(recording_to_playlist_item(playlist_id, video_id))
        .add_part("id")
        .add_part("snippet")
        .doit()
        .await
        .map(|_| ())
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

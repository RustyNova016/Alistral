use google_youtube3::api::Playlist;
use google_youtube3::api::PlaylistItem;
use google_youtube3::api::PlaylistItemSnippet;
use google_youtube3::api::PlaylistSnippet;
use google_youtube3::api::ResourceId;

use crate::database::recording;
use crate::models::playlist_stub::PlaylistStub;
use crate::models::services::youtube::Youtube;
use crate::Client;

impl Youtube {
    pub async fn create_playlist(
        client: &Client,
        playlist: PlaylistStub,
    ) -> Result<String, crate::Error> {
        let response = client
            .youtube_client()?
            .playlists()
            .insert(playlist.clone().to_yt_playlist())
            .add_part("id")
            .add_part("snippet")
            .doit()
            .await?;

        let playlist_id = response.1.id.expect("No id returned"); //TODO: Proper error

        for recording in playlist.recordings {
            //TODO: Check number of recordings missing
            //TODO: User overwrite
            let Some(video_id) = Self::get_or_query(client, recording, None).await? else {
                continue;
            };

            client
                .youtube_client()?
                .playlist_items()
                .insert(recording_to_playlist_item(playlist_id.clone(), video_id))
                .add_part("id")
                .add_part("snippet")
                .doit()
                .await?;
        }

        Ok(playlist_id)
    }
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

use listenbrainz::raw::request::PlaylistCreate;
use listenbrainz::raw::request::PlaylistCreatePlaylist;
use listenbrainz::raw::request::PlaylistCreatePlaylistExtension;
use listenbrainz::raw::request::PlaylistCreatePlaylistExtensionInner;
use listenbrainz::raw::request::PlaylistCreatePlaylistTrack;

use crate::models::playlist_stub::PlaylistStub;
use crate::InterzicClient;

pub struct Listenbrainz;

pub mod playlists;

impl Listenbrainz {
    pub async fn create_playlist(
        client: &InterzicClient,
        playlist: PlaylistStub,
        username: String,
        token: &str,
    ) -> Result<String, crate::Error> {
        Ok(client
            .listenbrainz_client()?
            .playlist_create(token, Self::into_jspf(playlist, username))?
            .playlist_mbid)
    }

    pub fn into_jspf(playlist: PlaylistStub, username: String) -> PlaylistCreate {
        PlaylistCreate {
            playlist: PlaylistCreatePlaylist {
                title: playlist.title,
                track: playlist
                    .recordings
                    .into_iter()
                    .filter_map(|r| r.mbid)
                    .map(|id| PlaylistCreatePlaylistTrack {
                        identifier: format!("https://musicbrainz.org/recording/{id}"),
                    })
                    .collect(),
                extension: PlaylistCreatePlaylistExtension {
                    musicbrainz: PlaylistCreatePlaylistExtensionInner {
                        created_for: None,
                        creator: Some(username),
                        collaborators: Vec::new(),
                        copied_from: None,
                        copied_from_deleted: None,
                        //public: self.public,
                        public: true, //TODO: #521 Allow setting playlist visibility
                        last_modified_at: None,
                        additional_metadata: None,
                    },
                },
                annotation: Some(playlist.description),
            },
        }
    }

    pub fn import_playlist(
        client: &InterzicClient,
        playlist_id: &str,
    ) -> Result<PlaylistStub, crate::Error> {
        Ok(client
            .listenbrainz_client()?
            .get_playlist(playlist_id)
            .map(|res| PlaylistStub::from(res.playlist))?)
    }
}

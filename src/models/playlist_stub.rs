use listenbrainz::raw::request::PlaylistCreate;
use listenbrainz::raw::request::PlaylistCreatePlaylist;
use listenbrainz::raw::request::PlaylistCreatePlaylistExtension;
use listenbrainz::raw::request::PlaylistCreatePlaylistExtensionInner;
use listenbrainz::raw::request::PlaylistCreatePlaylistTrack;
use listenbrainz::raw::response::PlaylistCreateResponse;
use macon::Builder;

use crate::api::listenbrainz::LISTENBRAINZ_CLIENT;

#[derive(Clone, Builder)]
pub struct PlaylistStub {
    title: String,
    description: Option<String>,
    username: Option<String>,
    public: bool,
    tracks: Vec<String>,
}

impl PlaylistStub {
    pub fn new(
        title: String,
        username: Option<String>,
        public: bool,
        tracks: Vec<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            title,
            username,
            public,
            tracks,
            description,
        }
    }

    pub fn into_jspf(self) -> PlaylistCreate {
        PlaylistCreate {
            playlist: PlaylistCreatePlaylist {
                title: self.title,
                track: self
                    .tracks
                    .iter()
                    .map(|id| PlaylistCreatePlaylistTrack {
                        identifier: format!("https://musicbrainz.org/recording/{id}"),
                    })
                    .collect(),
                extension: PlaylistCreatePlaylistExtension {
                    musicbrainz: PlaylistCreatePlaylistExtensionInner {
                        created_for: None,
                        creator: self.username,
                        collaborators: Vec::new(),
                        copied_from: None,
                        copied_from_deleted: None,
                        public: self.public,
                        last_modified_at: None,
                        additional_metadata: None,
                    },
                },
                annotation: self.description,
            },
        }
    }

    pub async fn send(self, token: &str) -> Result<PlaylistCreateResponse, crate::Error> {
        Ok(LISTENBRAINZ_CLIENT.playlist_create(token, self.into_jspf())?)
    }
}

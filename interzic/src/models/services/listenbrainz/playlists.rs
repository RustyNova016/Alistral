use listenbrainz::raw::jspf;
use listenbrainz::raw::jspf::Track;
use musicbrainz_rs::utils::parse_mbid;

use crate::models::messy_recording::MessyRecording;
use crate::models::playlist_stub::PlaylistStub;

impl From<jspf::PlaylistInfo> for PlaylistStub {
    fn from(value: jspf::PlaylistInfo) -> Self {
        PlaylistStub {
            title: value.title,
            description: value.annotation.unwrap_or_default(),
            recordings: value.track.into_iter().map(MessyRecording::from).collect(),
        }
    }
}

impl From<Track> for MessyRecording {
    fn from(value: Track) -> Self {
        Self {
            id: 0,
            artist_credits: value.creator,
            title: value.title,
            release: Some(value.album),
            mbid: value
                .identifier
                .into_iter()
                .filter_map(|s| parse_mbid(&s))
                .next(),
        }
    }
}

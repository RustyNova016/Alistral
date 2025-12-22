use crate::models::messy_recording::MessyRecording;
use crate::models::services::subsonic::SubsonicClient;

impl SubsonicClient {
    /// Find a recording in the subsonic server. This return the external id on the server
    pub async fn find_recording(&self, recording: &MessyRecording) -> Option<String> {
        let res = self
            .inner_client
            .search3(
                format!("{} {}", recording.title, recording.artist_credits),
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(20),
                Some(0),
                None::<String>,
            )
            .await
            .unwrap();

        // Search by MBID
        if let Some(target_mbid) = &recording.mbid {
            if let Some(song) = res.song.iter().find(|res| {
                res.music_brainz_id
                    .as_ref()
                    .is_some_and(|mbid| mbid == target_mbid)
            }) {
                return Some(song.id.to_owned());
            }
        }

        // Try to associate by recording name
        if let Some(song) = res.song.iter().find(|res| res.name == recording.title) {
            return Some(song.id.to_owned());
        }

        res.song.first().map(|song| song.id.to_owned())
    }
}

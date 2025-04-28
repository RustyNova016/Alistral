use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PopularityRecordingResponseItem {
    //TODO: Replace with listenbrainz_rs's
    pub recording_mbid: String,
    pub total_listen_count: Option<u64>,
    pub total_user_count: Option<u64>,
}

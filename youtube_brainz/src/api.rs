use crate::client::Client;
use crate::models::messy_recording::MessyRecording;

impl Client {
    pub async fn get_recording_yt_id(
        &self,
        recording: MessyRecording,
    ) -> Result<Option<String>, crate::Error> {
        let result = self.youtube_client
            .search()
            .list(&vec!["id,snippet".to_string()])
            .max_results(10)
            .q(&recording.to_string())
            .safe_search("none")
            .add_type("video")
            .doit()
            .await?
            .1;

        Ok(result
            .items
            .and_then(|results| results.into_iter().filter_map(|item| item.id).next())
            .and_then(|id| id.video_id))
    }
}


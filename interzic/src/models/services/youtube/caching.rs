use crate::InterzicClient;
use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
use crate::models::services::youtube::Youtube;

impl Youtube {
    pub async fn save_ext_id(
        client: &InterzicClient,
        ext_id: String,
        recording_id: i64,
        user_overwrite: Option<String>,
    ) -> Result<Option<ExternalId>, crate::Error> {
        let ext_id = ExternalId {
            id: 0,
            ext_id,
            recording_id,
            service: "youtube".to_string(),
            user_overwrite: user_overwrite.unwrap_or_default(),
        };

        return Ok(Some(ext_id.upsert(&client.database_client).await?));
    }

    pub async fn save_url(
        client: &InterzicClient,
        url: &str,
        recording: &MessyRecording,
    ) -> Result<Option<ExternalId>, crate::Error> {
        let (ext_id, service) = if let Some(id) = Youtube::extract_id_from_url(url) {
            (id, "youtube".to_string())
        } else {
            return Ok(None);
        };

        let id = ExternalId {
            id: 0,
            recording_id: recording.id,
            ext_id,
            service,
            user_overwrite: "".to_string(),
        };

        let id = id.upsert(&client.database_client).await?;
        Ok(Some(id))
    }
}

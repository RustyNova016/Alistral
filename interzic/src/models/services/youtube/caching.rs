use crate::InterzicClient;
use crate::models::external_id::ExternalId;
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
}

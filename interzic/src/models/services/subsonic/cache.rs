use crate::InterzicClient;
use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
use crate::models::services::subsonic::SubsonicClient;

impl SubsonicClient {
    /// Save the mapping between a messy recording and the external id on the external service
    pub async fn save_external_id(
        &self,
        client: &InterzicClient,
        ext_id: String,
        recording_id: i64,
        user_overwrite: Option<String>,
    ) -> Result<Option<ExternalId>, crate::Error> {
        let ext_id = ExternalId {
            id: 0,
            ext_id,
            recording_id,
            service: self.service_name(),
            user_overwrite: user_overwrite.unwrap_or_default(),
        };

        return Ok(Some(ext_id.upsert(&client.database_client).await?));
    }

    /// Get the external id from this service saved in the database
    pub async fn get_saved(
        &self,
        client: &InterzicClient,
        recording_id: i64,
        user_overwrite: Option<String>,
    ) -> Result<Option<String>, crate::Error> {
        Ok(ExternalId::find(
            &client.database_client,
            recording_id,
            self.service_name(),
            user_overwrite,
        )
        .await?)
    }

    /// Get the external id from the database or ask the subsonic server
    pub async fn get_of_fetch(
        &self,
        client: &InterzicClient,
        messy_recording: &MessyRecording,
        user_overwrite: Option<String>,
    ) -> Result<Option<String>, crate::Error> {
        if let Some(id) = self
            .get_saved(client, messy_recording.id, user_overwrite.clone())
            .await?
        {
            return Ok(Some(id));
        }

        let Some(ext_id) = self.find_recording(&messy_recording).await else {
            return Ok(None);
        };

        self.save_external_id(client, ext_id.clone(), messy_recording.id, user_overwrite)
            .await?;

        Ok(Some(ext_id))
    }
}

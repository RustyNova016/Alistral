use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::relations::RelationContent;
use musicbrainz_rs::Fetch;

use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
use crate::models::services::youtube::Youtube;
use crate::InterzicClient;

pub struct Musicbrainz;

impl Musicbrainz {
    pub async fn fetch_and_save_urls(
        client: &InterzicClient,
        recording: &MessyRecording,
    ) -> Result<(), crate::Error> {
        let result = Recording::fetch()
            .id(recording
                .mbid
                .as_ref()
                .ok_or(crate::Error::MissingRequiredMBIDError())?)
            .with_url_relations()
            .execute_with_client(client.musicbrainz_rs_client()?)
            .await?;

        for rel in result.relations.unwrap_or_else(Vec::new) {
            if let RelationContent::Url(val) = rel.content {
                Self::save_url(client, &val.resource, recording).await?;
            }
        }

        Ok(())
    }

    async fn save_url(
        client: &InterzicClient,
        url: &str,
        recording: &MessyRecording,
    ) -> Result<(), crate::Error> {
        let (ext_id, service) = if let Some(id) = Youtube::extract_id_from_url(url) {
            (id, "youtube".to_string())
        } else {
            return Ok(());
        };

        let id = ExternalId {
            id: 0,
            recording_id: recording.id,
            ext_id,
            service,
            user_overwrite: "".to_string(),
        };

        id.upsert(&client.database_client).await?;
        Ok(())
    }
}

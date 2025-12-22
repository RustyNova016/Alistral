pub mod caching;
use musicbrainz_rs::Fetch;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::InterzicClient;
use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
#[cfg(feature = "youtube")]
use crate::models::services::youtube::Youtube;

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
        #[cfg_attr(not(feature = "youtube"), allow(unused_variables))] client: &InterzicClient,
        #[cfg_attr(not(feature = "youtube"), allow(unused_variables))] url: &str,
        #[cfg_attr(not(feature = "youtube"), allow(unused_variables))] recording: &MessyRecording,
    ) -> Result<Option<ExternalId>, crate::Error> {
        #[cfg(feature = "youtube")]
        if let Some(id) = Youtube::save_url(client, url, recording).await? {
            return Ok(Some(id));
        }

        Ok(None)
    }
}

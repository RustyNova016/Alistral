use google_youtube3::common::url;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::entity::relations::RelationContent;
use musicbrainz_rs::entity::url::Url;
use musicbrainz_rs::Fetch;

use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
use crate::Client;

pub struct Musicbrainz;

impl Musicbrainz {
    pub async fn fetch_and_save_urls(
        client: &Client,
        recording: &MessyRecording,
    ) -> Result<(), crate::Error> {
        let result = Recording::fetch().id(mbid).with_url_relations().execute_with_client(client).await?;

        for rel in result.relations.unwrap_or_else(Vec::new) {
            match rel.content {
                RelationContent::Url(val) => {},
                _ => {}
            }
        }

        Ok(())
    }

    fn save_url(client: &Client, url: &str, recording: &MessyRecording) {
        let ext_id = ""; //TODO
        let service = "service";

        let id = ExternalId {
            id: 0,
            recording_id: recording.id,
            ext_id: todo!(),
            service: todo!(),
            user_overwrite: None,
            
        }
    }
}
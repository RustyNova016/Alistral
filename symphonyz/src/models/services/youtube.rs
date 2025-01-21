use crate::models::external_id::ExternalId;
use crate::models::messy_recording::MessyRecording;
use crate::models::services::musicbrainz::Musicbrainz;
use crate::utils::regexes::YOUTUBE_URL_ID_REGEX;
use crate::Client;

pub struct Youtube;

impl Youtube {
    pub async fn query_recording_id(
        client: &Client,
        recording: &MessyRecording,
    ) -> Result<Option<String>, crate::Error> {
        let result = client
            .youtube_client()?
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

    pub fn extract_id_from_url(url: &str) -> Option<String> {
        if let Some(capt) = YOUTUBE_URL_ID_REGEX.captures_iter(url).next() {
            return capt.get(7).map(|m| m.as_str().to_string()); // Capture group 7 is the one containing the id
        }

        None
    }

    pub async fn get_or_query(
        client: Client,
        recording: MessyRecording,
        user_overwrite: Option<String>,
    ) -> Result<Option<String>, crate::Error> {
        if let Some(id) = get_cached(&client, &recording, user_overwrite.clone()).await? {
            return Ok(Some(id));
        }

        Musicbrainz::fetch_and_save_urls(&client, &recording).await?;

        if let Some(id) = get_cached(&client, &recording, user_overwrite).await? {
            return Ok(Some(id));
        }

        let id = Self::query_recording_id(&client, &recording).await?;

        if let Some(id) = id {
            let ext_id = ExternalId {
                id: 0,
                ext_id: id.clone(),
                recording_id: recording.id,
                service: "youtube".to_string(),
                user_overwrite: None,
            };
            ext_id.upsert(&client.database_client).await?;
            return Ok(Some(id));
        }
        Ok(None)
    }
}

async fn get_cached(
    client: &Client,
    recording: &MessyRecording,
    user_overwrite: Option<String>,
) -> Result<Option<String>, crate::Error> {
    Ok(ExternalId::find(
        &client.database_client,
        recording.id,
        "youtube".to_string(),
        user_overwrite,
    )
    .await?)
}

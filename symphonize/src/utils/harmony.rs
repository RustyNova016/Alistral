use musicbrainz_db_lite::Recording;
use musicbrainz_db_lite::Release;
use musicbrainz_db_lite::models::musicbrainz::recording::relations::releases::RecordingReleasesDBRel;
use musicbrainz_db_lite::models::shared_traits::db_relation::EntityActiveURLDBRel;

use crate::SymphonyzeClient;
use crate::utils::link_supported_by_harmony;

pub async fn get_harmony_compatible_release_for_recording(
    client: &SymphonyzeClient,
    recording: &Recording,
) -> Result<Option<Release>, crate::Error> {
    let releases = recording
        .get_related_entity_or_fetch_as_task::<RecordingReleasesDBRel>(&client.mb_database)
        .await?;

    for release in releases {
        if is_release_harmony_compatible(client, &release).await? {
            return Ok(Some(release));
        }
    }

    Ok(None)
}

pub async fn is_release_harmony_compatible(
    client: &SymphonyzeClient,
    release: &Release,
) -> Result<bool, crate::Error> {
    let release_urls = release
        .get_related_entity_or_fetch_as_task::<EntityActiveURLDBRel>(&client.mb_database)
        .await?;

    for url in release_urls {
        if link_supported_by_harmony(&url.ressource) {
            return Ok(true);
        }
    }

    Ok(false)
}

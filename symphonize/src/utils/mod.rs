pub mod regex;
use musicbrainz_db_lite::Release;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use musicbrainz_db_lite::models::shared_traits::db_relation::EntityURLDBRel;

use crate::SymphonyzeClient;
use crate::utils::regex::HARMONY_SEED_URL_REGEX;

pub fn formater(client: &SymphonyzeClient) -> MusicbrainzFormater {
    MusicbrainzFormater {
        artist_credits: true,
        listenbrainz_link: false,
        client: client.mb_database.clone(),
    }
}

pub async fn is_release_harmony_compatible(
    client: &SymphonyzeClient,
    release: &Release,
) -> Result<bool, crate::Error> {
    if release.barcode.is_none() {
        return Ok(false);
    }

    let urls = release
        .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
        .await?;

    Ok(urls
        .into_iter()
        .any(|url| HARMONY_SEED_URL_REGEX.is_match(&url.ressource)))
}

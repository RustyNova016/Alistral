use musicbrainz_rs::Fetch;
use musicbrainz_rs::entity::artist::Artist as MBArtist;
use sqlx::SqliteConnection;
use tracing::instrument;

use crate::DBClient;
use crate::Error;
use crate::models::musicbrainz::artist::Artist;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;

impl FetchMBID<MBArtist> for Artist {
    #[instrument]
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MBArtist, musicbrainz_rs::GetRequestError> {
        MBArtist::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_event_relations()
            .with_genres()
            .with_rating()
            .with_recording_relations()
            .with_recordings()
            .with_release_groups()
            .with_release_relations()
            .with_releases()
            .with_releases_and_discids()
            .with_series_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .with_works()
            .with_medias()
            .execute_with_client(&client.musicbrainz_client)
            .await
    }
}

impl Artist {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        Self::fetch_and_save_with_conn(conn, client, mbid).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::musicbrainz::artist::Artist;
    use crate::tests::fixtures::default_client::test_mb_client;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_artist() {
        let client = test_mb_client();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        let test_values = vec![
            "d51d0c5b-8003-4b38-97a2-6400a5128784",
            "733c6e6f-0306-403e-9243-82b16a4f82d8", // Artist -> Artist + Artist -> Label
        ];

        for test in test_values {
            let value = Artist::get_or_fetch(conn, &client, test).await.unwrap();

            assert!(value.is_some_and(|r| r.full_update_date.is_some()))
        }
    }
}

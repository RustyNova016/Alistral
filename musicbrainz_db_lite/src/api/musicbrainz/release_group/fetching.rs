use musicbrainz_rs::Fetch;
use musicbrainz_rs::entity::release_group::ReleaseGroup as MBReleaseGroup;

use crate::DBClient;
use crate::Error;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;

impl FetchMBID<MBReleaseGroup> for ReleaseGroup {
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MBReleaseGroup, musicbrainz_rs::Error> {
        MBReleaseGroup::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artists()
            .with_genres()
            .with_medias()
            .with_ratings()
            .with_release_group_relations()
            .with_releases()
            .with_series_relations()
            .with_tags()
            .with_url_relations()
            .execute_with_client(&client.musicbrainz_client)
            .await
    }
}

impl ReleaseGroup {
    pub async fn fetch_and_save(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        Self::fetch_and_save_with_conn(conn, client, mbid).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::musicbrainz::release_group::ReleaseGroup;
    use crate::tests::fixtures::default_client::test_mb_client;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_work() {
        let client = test_mb_client();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        let test_values = vec![
            "96058b5d-19d1-403d-b289-c45e6f10f077", // RG -> Series + RG -> URL
        ];

        for test in test_values {
            let value = ReleaseGroup::get_or_fetch(conn, &client, test)
                .await
                .unwrap();

            assert!(value.is_some_and(|r| r.full_update_date.is_some()))
        }
    }
}

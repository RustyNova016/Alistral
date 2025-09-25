use musicbrainz_rs::Fetch;
use musicbrainz_rs::entity::work::Work as MBWork;

use crate::Error;
use crate::database::client::DBClient;
use crate::models::musicbrainz::work::Work;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;

impl FetchMBID<MBWork> for Work {
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MBWork, musicbrainz_rs::Error> {
        MBWork::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_ratings()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .with_label_relations()
            .with_recording_relations()
            .execute_with_client(&client.musicbrainz_client)
            .await
    }
}

impl Work {
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

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::work::Work;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_work() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        let test_values = vec![
            "a2014be6-cbdc-4616-9c94-36b41e99af6a", // Work -> Artist
            "1919e988-9619-45fc-a2dc-91dbf52a85c2", // Work -> Work
        ];

        for test in test_values {
            let value = Work::get_or_fetch(conn, &client, test).await.unwrap();

            assert!(value.is_some_and(|r| r.full_update_date.is_some()))
        }
    }
}

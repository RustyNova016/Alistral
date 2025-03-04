use musicbrainz_rs_nova::entity::work::Work as MBWork;
use musicbrainz_rs_nova::Fetch;

use crate::api::SaveToDatabase;
use crate::database::client::DBClient;
use crate::models::musicbrainz::work::Work;
use crate::Error;

impl Work {
    pub async fn fetch_and_save(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        let data = MBWork::fetch()
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
            .await;

        match data {
            Ok(data) => {
                let mut data = data.save(conn).await?;
                data.reset_full_update_date(conn).await?;

                Self::set_redirection(conn, mbid, data.id).await?;

                Ok(Some(data))
            }
            Err(musicbrainz_rs_nova::Error::NotFound(_)) => {
                // TODO: Set deleted
                Ok(None)
            }
            Err(err) => Err(err.into()),
        }
    }
}

impl SaveToDatabase for MBWork {
    type ReturnedData = Work;

    async fn save(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Self::ReturnedData, crate::Error> {
        Work::save_api_response_recursive(conn, self).await
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::work::Work;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_work() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

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

use musicbrainz_rs_nova::entity::release_group::ReleaseGroup as MBReleaseGroup;
use musicbrainz_rs_nova::Fetch;

use crate::api::SaveToDatabase;
use crate::database::client::DBClient;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::Error;

impl ReleaseGroup {
    pub async fn fetch_and_save(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        let data = MBReleaseGroup::fetch()
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

impl SaveToDatabase for MBReleaseGroup {
    type ReturnedData = ReleaseGroup;

    async fn save(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Self::ReturnedData, crate::Error> {
        ReleaseGroup::save_api_response_recursive(conn, self).await
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::release_group::ReleaseGroup;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_work() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

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

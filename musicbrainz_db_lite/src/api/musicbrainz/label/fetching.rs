use musicbrainz_rs_nova::{entity::label::Label as MBLabel, Fetch};

use crate::database::client::DBClient;
use crate::{api::SaveToDatabase, models::musicbrainz::label::Label};

impl Label {
    // TODO: #51 Fix missing relations
    pub async fn fetch_and_save(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, crate::Error> {
        let data = MBLabel::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_medias()
            .with_ratings()
            .with_recording_relations()
            .with_release_relations()
            .with_releases()
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

impl SaveToDatabase for MBLabel {
    type ReturnedData = Label;

    async fn save(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Self::ReturnedData, crate::Error> {
        Label::save_api_response_recursive(conn, self).await
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::label::Label;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_label() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.acquire_guarded().await;

        // Test values. Feel free to add edge cases here
        let test_values = vec![
            "b10497d9-68c2-4f58-a9ae-8ba7b15d3e09",
            "ecc4aa34-753b-4031-9a12-53a27ec94955", // Label -> Artist + Label -> Serie
        ];

        for test in test_values {
            let value = Label::get_or_fetch(conn, &client, test).await.unwrap();

            assert!(value.is_some_and(|r| r.full_update_date.is_some()))
        }
    }
}

use crate::database::client::DBClient;
use crate::{
    api::SaveToDatabase,
    models::musicbrainz::release::{Release, Track},
    Error,
};
use musicbrainz_rs_nova::{entity::release::Release as MBRelease, Fetch};
use sqlx::Connection;
use sqlx::SqliteConnection;

impl Release {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        let data = MBRelease::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_credits()
            .with_artist_relations()
            .with_artists()
            .with_genres()
            .with_labels()
            .with_ratings()
            .with_recording_level_relations()
            .with_recordings()
            .with_release_groups()
            .with_tags()
            .with_url_relations()
            .with_work_level_relations()
            .with_work_relations()
            .execute_with_client(&client.musicbrainz_client)
            .await;

        match data {
            Ok(data) => {
                let mut trans = conn.begin().await?;
                let mut data = data.save(&mut trans).await?;
                data.reset_full_update_date(&mut trans).await?;

                Self::set_redirection(&mut trans, mbid, data.id).await?;
                trans.commit().await?;

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

impl SaveToDatabase for MBRelease {
    type ReturnedData = Release;

    async fn save(self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, crate::Error> {
        Release::save_api_response_recursive(conn, self).await
    }
}

impl Track {
    pub async fn refetch(
        &self,
        _conn: &mut sqlx::SqliteConnection,
        _client: &crate::DBClient,
    ) -> Result<Self, crate::Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use musicbrainz_db_lite_schema::create_and_migrate;

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::recording::Recording;
    use crate::models::musicbrainz::release::Release;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.acquire_guarded().await;
        create_and_migrate(conn).await.unwrap();

        // Test values. Feel free to add edge cases here
        let test_values = vec!["daf6e333-b491-490a-9444-8888cb08b141"];

        for test in test_values {
            let value = Release::get_or_fetch(conn, &client, test).await.unwrap();

            assert!(value.is_some_and(|r| r.full_update_date.is_some()))
        }
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_full_insert_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.acquire_guarded().await;

        // Test values. Feel free to add edge cases here
        // (Recording, Release)
        let test_values = vec![(
            "c6f1752b-a8b6-45a8-bdef-16fa9c859c81",
            "daf6e333-b491-490a-9444-8888cb08b141",
        )];

        for (recording_id, release_id) in test_values {
            // Get the recording to partially pull release info
            Recording::fetch_and_save(conn, &client, recording_id)
                .await
                .unwrap();

            let mut release = Release::get_or_fetch(conn, &client, release_id)
                .await
                .unwrap()
                .unwrap();

            assert!(release.full_update_date.is_none());

            release.refetch_and_load(conn, &client).await.unwrap();

            assert!(release.full_update_date.is_some());
        }
    }
}

use crate::database::client::DBClient;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;
use crate::{
    models::musicbrainz::release::{Release, Track},
    Error,
};
use musicbrainz_rs_nova::{entity::release::Release as MBRelease, Fetch};
use sqlx::SqliteConnection;

impl FetchMBID<MBRelease> for Release {
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MBRelease, musicbrainz_rs_nova::Error> {
        MBRelease::fetch()
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
            .await
    }
}

impl Release {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        Self::fetch_and_save_with_conn(conn, client, mbid).await
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
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::recording::Recording;
    use crate::models::musicbrainz::release::Release;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();
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
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

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

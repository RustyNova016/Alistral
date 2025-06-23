use musicbrainz_rs::{Fetch, entity::label::Label as MBLabel};

use crate::database::client::DBClient;
use crate::models::musicbrainz::label::Label;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;

impl FetchMBID<MBLabel> for Label {
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MBLabel, musicbrainz_rs::Error> {
        MBLabel::fetch()
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
            .await
    }
}

impl Label {
    pub async fn fetch_and_save(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, crate::Error> {
        Self::fetch_and_save_with_conn(conn, client, mbid).await
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::label::Label;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_label() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

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

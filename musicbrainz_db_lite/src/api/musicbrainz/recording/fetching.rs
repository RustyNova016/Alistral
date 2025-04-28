use crate::database::client::DBClient;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;
use crate::{Error, models::musicbrainz::recording::Recording};
use musicbrainz_rs_nova::{Fetch, entity::recording::Recording as MSRecording};
use sqlx::SqliteConnection;

impl FetchMBID<MSRecording> for Recording {
    async fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> Result<MSRecording, musicbrainz_rs_nova::Error> {
        MSRecording::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artists()
            .with_genres()
            .with_isrcs()
            .with_ratings()
            .with_releases()
            .with_tags()
            // relations
            .with_area_relations()
            .with_artist_relations()
            .with_event_relations()
            .with_genre_relations()
            .with_instrument_relations()
            .with_label_relations()
            .with_place_relations()
            .with_recording_relations()
            .with_recording_relations()
            .with_release_relations()
            .with_series_relations()
            .with_url_relations()
            .with_work_relations()
            // Extra relations
            .with_work_level_relations()
            .with_medias()
            .execute_with_client(&client.musicbrainz_client)
            .await
    }
}

impl Recording {
    /// Fetch a recording with all relationships. Then save to the db
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Recording>, Error> {
        Self::fetch_and_save_with_conn(conn, client, mbid).await
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::recording::Recording;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_recording() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        let test_values = vec![
            "5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef",
            "2d1a7579-10dc-471b-a758-5f63f9d2e5dd", // Artist -> Recording + Recording -> Recording
        ];

        for test in test_values {
            let value = Recording::get_or_fetch(conn, &client, test)
                .await
                .unwrap()
                .expect("The recording should be there");

            assert!(value.full_update_date.is_some());

            let credits = value
                .get_artist_credits_or_fetch(conn, &client)
                .await
                .unwrap();
            assert!(!credits.1.is_empty())
        }
    }
}

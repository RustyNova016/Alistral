use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use musicbrainz_rs_nova::Fetch;
use sqlx::SqliteConnection;
use tracing::debug;

use crate::api::SaveToDatabase;
use crate::database::client::DBClient;
use crate::models::musicbrainz::artist::Artist;
use crate::Error;

impl Artist {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        client: &DBClient,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        debug!(mbid = mbid);

        // TODO: #51 Fix missing relations
        let data = MBArtist::fetch()
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

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::musicbrainz::artist::Artist;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_artist() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();

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

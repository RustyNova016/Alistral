use std::collections::HashMap;

use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::ArtistCredit;
use crate::DBRelation;
use crate::FetchAsComplete as _;
use crate::Track;
use crate::models::musicbrainz::isrc::ISRC;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::track::relations::TrackRecordingDBRel;
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;
use crate::models::shared_traits::db_relation::RecordingISRCDBRel;
use crate::utils::sqlx_utils::entity_relations::{JoinCollection, JoinRelation};

use super::Recording;

pub mod artist;
pub mod crawler;
pub mod releases;
pub mod urls;
pub mod work;

impl Recording {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_releases_or_fetch(
        &self,
        conn: &mut SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Release>, crate::Error> {
        // First, make sure all the releases of the recording are in the database
        self.fetch_as_complete_with_conn(conn, client).await?;

        // Next, get all the releases
        Ok(sqlx::query_as(
            r#"SELECT
                    releases.*
                FROM
                    releases
                    INNER JOIN medias ON medias.`release` = releases.id
                    INNER JOIN tracks ON tracks.media = medias.id
                    INNER JOIN recordings ON recordings.id = tracks.recording
                WHERE
                    recordings.id = ?"#,
        )
        .bind(self.id)
        .fetch_all(conn)
        .await?)
    }

    /// Get a all the releases of a list of recordings.
    ///
    /// ⚠️ The recordings must all be fetched before. A `debug_assert` will block in case of, but won't trigger in production
    pub async fn get_releases_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        recordings: &'r [&'r Recording],
    ) -> Result<HashMap<i64, (&'r &'r Recording, Vec<Release>)>, crate::Error> {
        //#[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = recordings.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Release>> = sqlx::query_as(
            "
            SELECT
                recordings.id as original_id,
                releases.*
            FROM
                recordings
                INNER JOIN tracks ON recordings.id = tracks.recording
                INNER JOIN medias ON tracks.media = medias.id
                INNER JOIN releases ON medias.`release` = releases.id
            WHERE
                recordings.id IN (
                    SELECT
                        value
                    FROM
                        JSON_EACH(?)
                )
        ",
        )
        .bind(id_string)
        .fetch_all(conn)
        .await?;

        Ok(JoinCollection::from(joins).into_hashmap(recordings, |id, value| &value.id == id))
    }
}

impl DBRelation<ArtistCreditDBRel> for Recording {
    type ReturnedType = ArtistCredit;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON recordings.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit"
    }
}

impl DBRelation<RecordingISRCDBRel> for Recording {
    type ReturnedType = ISRC;

    fn get_join_statement() -> &'static str {
        "INNER JOIN `isrcs` ON recordings.id = `isrcs`.recording"
    }
}

impl DBRelation<TrackRecordingDBRel> for Recording {
    type ReturnedType = Track;

    fn get_join_statement() -> &'static str {
        "INNER JOIN tracks ON recordings.id = tracks.recording"
    }
}

#[cfg(test)]
mod tests {

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::recording::Recording;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_release_group_from_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        // (Recording MBID, Release MBID)
        let test_values = vec![(
            "543bb836-fb00-470a-8a27-25941fe0294c",
            "19d60a3e-0980-4ce9-bc3a-c72cb49ebd4c",
        )];

        for (left, right) in test_values {
            let value = Recording::get_or_fetch(conn, &client, left)
                .await
                .expect("Error during fetch")
                .expect("The release should exists");

            let right_value = value
                .get_releases_or_fetch(conn, &client)
                .await
                .expect("Error during fetching");

            println!("{right_value:#?}");

            right_value
                .iter()
                .find(|r| r.mbid == right)
                .expect("There should have a release matching the recording");
        }
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_original_mix_from_remix() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        // (Remix Recording MBID, Original Recording MBID)
        let test_values = vec![(
            "be4ac8b9-37eb-45cb-a2eb-d74f9f2ebc88",
            "497b48ed-0ec9-4ba2-822a-0fbed83dac36",
        )];

        for (left, right) in test_values {
            let value = Recording::get_or_fetch(conn, &client, left)
                .await
                .expect("Error during fetch")
                .expect("The release should exists");

            let right_value = value
                .get_recording_relations(conn)
                .await
                .expect("Error during fetching");

            let mut found = false;
            for relation in right_value {
                let related = relation.get_entity_1_as_left(conn).await.unwrap();
                println!("{related:#?}");
                if related.mbid == right {
                    found = true
                }
            }

            assert!(found);
        }
    }
}

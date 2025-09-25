use std::collections::HashMap;

use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::DBRelation;
use crate::models::musicbrainz::work::Work;
use crate::models::shared_traits::db_relation::RecordingWorkDBRel;
use crate::utils::sqlx_utils::entity_relations::{JoinCollection, JoinRelation};

use super::Recording;

impl Recording {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_works_or_fetch(
        &self,
        conn: &mut SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Work>, crate::Error> {
        // First, make sure all the work of the recording are in the database
        self.fetch_if_incomplete(conn, client).await?;

        // Next, get all the works
        Ok(sqlx::query_as(
            r#"SELECT
                    works.*
                FROM
                    works
                    INNER JOIN l_recordings_works as rel ON works.id = rel.entity1
                    INNER JOIN recordings ON rel.entity0 = recordings.id
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
    pub async fn get_works_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        recordings: &'r [&'r Recording],
    ) -> Result<HashMap<i64, (&'r &'r Recording, Vec<Work>)>, crate::Error> {
        //#[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = recordings.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Work>> = sqlx::query_as(
            "
            SELECT
                recordings.id as original_id,
                works.*
            FROM
                works
                INNER JOIN l_recordings_works as rel ON works.id = rel.entity1
                INNER JOIN recordings ON rel.entity0 = recordings.id
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

impl DBRelation<RecordingWorkDBRel> for Recording {
    type ReturnedType = Work;

    fn get_join_statement() -> &'static str {
        "INNER JOIN l_recordings_works as rel ON rel.entity0 = recordings.id
        INNER JOIN works ON works.id = rel.entity1"
    }
}

#[cfg(test)]
mod tests {
    use crate::database::client::DBClient;
    use crate::models::musicbrainz::recording::Recording;
    use crate::utils::tests::RelationAssertion;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_recordings_from_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        // (Release, Recording)
        let test_values = vec![RelationAssertion {
            left_id: "0e12c33d-20f8-4daa-97ac-5ec21411c1b0",
            right_id: "2dceb174-32cc-471c-ac0a-fc237bb9f257",
        }];

        for assertion in &test_values {
            let value = Recording::get_or_fetch(conn, &client, assertion.left_id)
                .await
                .expect("Error during fetch")
                .expect("The recording should exists");

            let right_values = value
                .get_works_or_fetch(conn, &client)
                .await
                .expect("Error during fetching");

            assertion.assert_has_element_with_mbid(&right_values);

            let var_name = vec![&value];
            let right_values = Recording::get_works_as_batch(conn, &var_name)
                .await
                .expect("Error during fetching");

            RelationAssertion::assert_batch_join_has_relation(&test_values, &right_values);
        }
    }
}

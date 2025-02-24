use std::collections::HashMap;

use itertools::Itertools;

use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

impl Release {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_recordings_or_fetch(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Recording>, crate::Error> {
        // First, make sure the entity is in the database
        self.fetch_if_incomplete(conn, client).await?;

        // Next, get all the works
        Ok(sqlx::query_as(
            r#"SELECT
                    recordings.*
                FROM
                    releases
                    INNER JOIN medias ON medias.`release` = releases.id
                    INNER JOIN tracks ON tracks.media = medias.id
                    INNER JOIN recordings ON recordings.id = tracks.recording
                WHERE
                    releases.id = ?"#,
        )
        .bind(self.id)
        .fetch_all(conn)
        .await?)
    }

    /// Get a all the releases of a list of recordings.
    ///
    /// ⚠️ The releases must all be fetched before. A `debug_assert` will block in case of, but won't trigger in production
    pub async fn get_recordings_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        releases: &'r [&'r Release],
    ) -> Result<HashMap<i64, (&'r &'r Release, Vec<Recording>)>, crate::Error> {
        //#[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = releases.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Recording>> = sqlx::query_as(
            "
            SELECT
                releases.id as original_id,
                recordings.*
            FROM
                releases
                INNER JOIN medias ON medias.`release` = releases.id
                INNER JOIN tracks ON tracks.media = medias.id
                INNER JOIN recordings ON recordings.id = tracks.recording
            WHERE
                releases.id IN (
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

        Ok(JoinCollection::from(joins).into_hashmap(releases, |id, value| &value.id == id))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::client::DBClient;
    use crate::models::musicbrainz::release::Release;
    use crate::utils::tests::RelationAssertion;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_recordings_from_release() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.acquire_guarded().await;

        // Test values. Feel free to add edge cases here
        // (Release, Recording)
        let test_values = vec![RelationAssertion {
            left_id: "da8292ec-8ce2-4921-b9be-ea5b6e289d84",
            right_id: "6a880796-53c7-4c20-ba0f-730aa90423b8",
        }];

        for assertion in &test_values {
            let value = Release::get_or_fetch(conn, &client, assertion.left_id)
                .await
                .expect("Error during fetch")
                .expect("The release should exists");

            let right_values = value
                .get_recordings_or_fetch(conn, &client)
                .await
                .expect("Error during fetching");

            assertion.assert_has_element_with_mbid(&right_values);

            let var_name = vec![&value];
            let right_values = Release::get_recordings_as_batch(conn, &var_name)
                .await
                .expect("Error during fetching");

            RelationAssertion::assert_batch_join_has_relation(&test_values, &right_values);
        }
    }
}

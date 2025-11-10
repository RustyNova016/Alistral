use std::collections::HashMap;
use std::sync::Arc;

use itertools::Itertools;

use crate::DBClient;
use crate::DBRelation;
use crate::FetchAsComplete as _;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

/// Recording (1:M) -> Releases
pub struct ReleasesReleasesgroupDBRel;

impl DBRelation<ReleasesReleasesgroupDBRel> for Release {
    type ReturnedType = ReleaseGroup;

    fn get_join_statement() -> &'static str {
        "INNER JOIN releases ON release_groups.id = releases.release_group"
    }
}

impl Release {
    /// Get the release group associated to the release
    pub async fn get_release_group(
        &self,
        client: &Arc<DBClient>,
    ) -> Result<Vec<ReleaseGroup>, crate::Error> {
        self.get_related_entity_or_fetch_as_task::<ReleasesReleasesgroupDBRel>(client)
            .await
    }

    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_release_group_or_fetch(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<ReleaseGroup, crate::Error> {
        // First, make sure all the work of the recording are in the database
        self.fetch_as_complete_with_conn(conn, client).await?;

        // Next, get all the works
        Ok(sqlx::query_as(
            r#"SELECT
                    release_groups.*
                FROM
                    release_groups
                    INNER JOIN releases ON release_groups.id = releases.release_group
                WHERE
                    releases.id = ?"#,
        )
        .bind(self.id)
        .fetch_one(conn)
        .await?)
    }

    /// Get a all the releases of a list of recordings.
    ///
    /// ⚠️ The releases must all be fetched before. A `debug_assert` will block in case of, but won't trigger in production
    pub async fn get_release_groups_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        releases: &'r [&'r Release],
    ) -> Result<HashMap<i64, (&'r &'r Release, Vec<ReleaseGroup>)>, crate::Error> {
        //#[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = releases.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, ReleaseGroup>> = sqlx::query_as(
            "
            SELECT
                releases.id as original_id,
                release_groups.*
            FROM
                release_groups
                INNER JOIN releases ON release_groups.id = releases.release_group
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
    use crate::models::musicbrainz::release::Release;
    use crate::tests::fixtures::default_client::test_mb_client;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_release_group_from_release() {
        let client = test_mb_client();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        // Test values. Feel free to add edge cases here
        // (Release MBID, RG MBID)
        let test_values = vec![(
            "19d60a3e-0980-4ce9-bc3a-c72cb49ebd4c",
            "b2a5ae23-c656-4cb3-88e6-1d453595d4bc",
        )];

        for (left, right) in test_values {
            let value = Release::get_or_fetch(conn, &client, left)
                .await
                .expect("Error during fetch")
                .expect("The release should exists");

            let right_value = value
                .get_release_group_or_fetch(conn, &client)
                .await
                .expect("Error during fetching");

            assert_eq!(right_value.mbid, right);
        }
    }
}

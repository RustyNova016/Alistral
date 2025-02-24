use std::collections::HashMap;

use itertools::Itertools;

use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

impl Listen {
    /// Get the messybrainz data of the listen
    pub async fn get_messybrainz_data(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<MessybrainzSubmission, crate::Error> {
        Ok(sqlx::query_as(
                    r#"
                    SELECT
                        messybrainz_submission.*
                    FROM
                        listens
                        INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                    WHERE
                        listens.id = ?"#,
                )
                .bind(self.id)
                .fetch_one(conn)
                .await?)
    }

    /// Get a all the messybrainz data of a list of listens.
    pub async fn get_messybrainz_data_from_listen_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        entities: &'r [&'r Listen],
    ) -> Result<HashMap<i64, (&'r &'r Listen, Vec<MessybrainzSubmission>)>, crate::Error> {
        let ids = entities.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, MessybrainzSubmission>> = sqlx::query_as(
                "
                SELECT
                    listens.id as original_id,
                    messybrainz_submission.*
                FROM
                    listens
                    INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                WHERE
                    listens.id IN (
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

        Ok(JoinCollection::from(joins).into_hashmap(entities, |id, value| &value.id == id))
    }
}

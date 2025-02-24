use std::collections::HashMap;

use itertools::Itertools;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::recording::Recording;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

impl Recording {
    /// Get the artists of the recording, and fetch them if necessary.
    pub async fn get_artists_or_fetch(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Artist>, crate::Error> {
        // First, make sure all the work of the recording are in the database
        self.fetch_if_incomplete(conn, client).await?;

        // Next, get all the works
        Ok(sqlx::query_as(
                r#"SELECT
                        artists.*
                    FROM
                        recordings
                        INNER JOIN artist_credits ON recordings.artist_credit = artist_credits.id
                        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
                        INNER JOIN artists_gid_redirect ON artist_credits_item.artist_gid = artists_gid_redirect.gid
                        INNER JOIN artists ON artists_gid_redirect.new_id = artists.id
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
    pub async fn get_artist_from_credits_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        recordings: &'r [&'r Recording],
    ) -> Result<HashMap<i64, (&'r &'r Recording, Vec<Artist>)>, crate::Error> {
        //#[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = recordings.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Artist>> = sqlx::query_as(
            "
            SELECT
                recordings.id as original_id,
                artists.*
            FROM
                recordings
                INNER JOIN artist_credits ON recordings.artist_credit = artist_credits.id
                INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
                INNER JOIN artists_gid_redirect ON artist_credits_item.artist_gid = artists_gid_redirect.gid
                INNER JOIN artists ON artists_gid_redirect.new_id = artists.id
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

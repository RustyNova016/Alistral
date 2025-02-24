use super::MessybrainzSubmission;

impl MessybrainzSubmission {
    /// Return all the `MessybrainzSubmission` that are mapped to a mbid (or its aliases) for an user
    pub async fn get_messybrainzs_from_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        user_id: i64,
    ) -> Result<Vec<MessybrainzSubmission>, crate::Error> {
        let result: Vec<MessybrainzSubmission> = sqlx::query_as(
            "
            SELECT
                messybrainz_submission.*
            FROM
                messybrainz_submission
                INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            WHERE
                new_id = (
                    SELECT
                        new_id
                    FROM
                        `recordings_gid_redirect`
                    WHERE
                        recordings_gid_redirect.gid = ?
                        AND deleted = 0
                )
                AND msid_mapping.user = ?
                AND deleted = 0
",
        ).bind(mbid).bind(user_id).fetch_all(conn).await?;

        Ok(result)
    }
}

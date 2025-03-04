use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;

impl MessybrainzSubmission {
    pub async fn get_listens_of_msid(
        conn: &mut sqlx::SqliteConnection,
        msid: &str,
    ) -> Result<Vec<Listen>, crate::Error> {
        Ok(sqlx::query_as!(
            Listen,
            "
        SELECT
            listens.*
        FROM
            messybrainz_submission
            INNER JOIN listens ON messybrainz_submission.msid = listens.recording_msid
        WHERE
            messybrainz_submission.msid = ?
",
            msid
        )
        .fetch_all(conn)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use listenbrainz::raw::Client;

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt;
    use crate::models::listenbrainz::listen::Listen;
    use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_listens_of_msid() {
        let client = DBClient::connect_in_memory_and_create().await.unwrap();
        let conn = &mut *client.connection.get_raw_connection().await.unwrap();
        let lb_client = Client::new();

        // Test values. Feel free to add edge cases here
        let test_values = vec![(
            1732782032,
            "RustyNova",
            "346532b6-dbec-4685-b20d-56a0257b351c",
        )];

        for (listened_at, user, msid) in test_values {
            let base_listen =
                Listen::fetch_listen_by_id(conn, &lb_client, listened_at, user, msid, 100)
                    .await
                    .unwrap()
                    .unwrap();

            let listens = MessybrainzSubmission::get_listens_of_msid(
                conn,
                "346532b6-dbec-4685-b20d-56a0257b351c",
            )
            .await
            .unwrap();

            assert!(listens.contains(&base_listen));
        }
    }
}

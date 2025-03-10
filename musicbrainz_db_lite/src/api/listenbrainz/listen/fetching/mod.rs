//TODO: Proper Rate Limiter
//pub mod rate_limiter;
pub mod api;
pub mod query;
use chrono::DateTime;
use chrono::Utc;
use listenbrainz::raw::Client;
use tracing::info;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::api::listenbrainz::listen_collection::SaveListenPayload;
use crate::models::listenbrainz::listen::Listen;
use crate::DBClient;
use crate::Error;

impl Listen {
    /// Fetch the latest listens for the provided user. If the user has no listens, it will do a full listen fetch.
    #[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn fetch_latest_listens_of_user(
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        user: &str,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Fetching listens");
        info!("Fetching latest listens of {}", user);

        let latest_listen_ts = Listen::get_latest_listen_of_user(&mut *conn, user)
            .await?
            .map(|v| v.listened_at);
        let mut pull_ts = Some(Utc::now().timestamp());

        // This loop has two possible states.
        // - Fresh dump:
        //     `latest_listen_ts` is none. We loop until `save_listen_payload_in_transaction` tell us it's over
        //
        // - Incremental dump:
        //     `latest_listen_ts` is set. We loop until pull_ts is before the latest listen
        while (latest_listen_ts.is_none() && pull_ts.is_some())
            || (latest_listen_ts.is_some_and(|a| pull_ts.is_some_and(|b| a <= b)))
        {
            info!(
                "Getting listens from before: {} ({})",
                DateTime::from_timestamp(pull_ts.unwrap(), 0).unwrap(),
                pull_ts.unwrap()
            );
            pull_ts = Listen::execute_listen_fetch(
                conn,
                &client.listenbrainz_client,
                user,
                pull_ts.unwrap(),
            )
            .await?;
        }

        Ok(())
    }

    /// Fetch listens for the user and save them in the database
    pub async fn execute_listen_fetch(
        conn: &mut sqlx::SqliteConnection,
        lb_client: &Client,
        user: &str,
        max_ts: i64,
    ) -> Result<Option<i64>, Error> {
        info!(
            "Getting listens from before: {} ({})",
            DateTime::from_timestamp(max_ts, 0).unwrap(),
            max_ts
        );
        let dump = lb_client.user_listens(user, None, Some(max_ts), Some(1000));

        match dump {
            Ok(val) => Ok(val
                .payload
                .save_listen_payload_in_transaction(conn, max_ts, 1000)
                .await?),

            #[cfg(feature = "timeout_continue")]
            Err(listenbrainz::Error::Http(_err)) => Ok(None),

            Err(err) => Err(err)?,
        }
    }

    /// Fetch a listen by it's id (listened_at, username, msid)
    #[cfg(feature = "timeout_continue")]
    pub async fn fetch_listen_by_id(
        conn: &mut sqlx::SqliteConnection,
        lb_client: &Client,

        listened_at: i64,
        user: &str,
        msid: &str,

        max_count: i64,
    ) -> Result<Option<Listen>, crate::Error> {
        #[cfg_attr(not(feature = "timeout_continue"), expect(unused_mut))]
        let mut fetch_count = max_count;

        while fetch_count != 0 {
            let dump = lb_client.user_listens(
                user,
                None,
                Some(listened_at + 1),
                Some(fetch_count.try_into().unwrap()),
            );

            match dump {
                Ok(results) => {
                    // Save the listens
                    results
                        .payload
                        .save_listen_payload_in_transaction(
                            conn,
                            listened_at + 1,
                            fetch_count.try_into().unwrap(),
                        )
                        .await?;

                    return Listen::get_by_unique_triplet(conn, listened_at, msid, user).await;
                }

                Err(listenbrainz::Error::Http(_err)) => fetch_count = fetch_count.div_euclid(2),

                Err(err) => Err(err)?,
            }
        }

        Err(crate::Error::ListenFetchingTimeout)
    }

    /// Fetch a listen by it's id (listened_at, username, msid)
    #[cfg(not(feature = "timeout_continue"))]
    pub async fn fetch_listen_by_id(
        conn: &mut sqlx::SqliteConnection,
        lb_client: &Client,

        listened_at: i64,
        user: &str,
        msid: &str,

        max_count: i64,
    ) -> Result<Option<Listen>, crate::Error> {
        let dump = lb_client.user_listens(
            user,
            None,
            Some(listened_at + 1),
            Some(max_count.try_into().unwrap()),
        );

        match dump {
            Ok(results) => {
                // Save the listens
                results
                    .payload
                    .save_listen_payload_in_transaction(
                        conn,
                        listened_at + 1,
                        max_count.try_into().unwrap(),
                    )
                    .await?;

                Listen::get_by_unique_triplet(conn, listened_at, msid, user).await
            }

            Err(listenbrainz::Error::Http(_err)) => Err(crate::Error::ListenFetchingTimeout)?,

            Err(err) => Err(err)?,
        }
    }
}

#[cfg(test)]
mod tests {
    use listenbrainz::raw::Client;

    use crate::database::client::DBClient;
    use crate::database::pool::DBLitePoolExt as _;
    use crate::models::listenbrainz::listen::Listen;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_fetch_listen_by_triplet() {
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
            Listen::fetch_listen_by_id(conn, &lb_client, listened_at, user, msid, 100)
                .await
                .unwrap()
                .unwrap();
        }
    }
}

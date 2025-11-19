use chrono::Duration;
use chrono::Utc;
use listenbrainz_rs::api::ListenBrainzAPI;
use listenbrainz_rs::client::ListenBrainzClient;
use snafu::ResultExt as _;
use sqlx::Acquire as _;

use crate::DBClient;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::listen::fetching::ConnectionSnafu;
use crate::models::listenbrainz::listen::fetching::LBApiRequestSnafu;
use crate::models::listenbrainz::listen::fetching::ListenFetchingError;
use crate::models::listenbrainz::listen::fetching::save_listens;

impl Listen {
    /// Fetch and save listens of an user incrementally
    pub async fn fetch_and_save_incremental(
        client: &DBClient,
        username: &str,
    ) -> Result<(), ListenFetchingError> {
        //TODO: Add the client to the main client
        let lbclient = ListenBrainzClient::new();

        let conn = &mut *client.database.get_conn().await.context(ConnectionSnafu)?;
        let latest_listen = Listen::get_latest_listen_of_user(conn, username).await?;

        // Offset by 3 days to catch some remappings
        let start = latest_listen
            .map(|l| (l.listened_at_as_datetime() - Duration::days(3)).timestamp() as u64);

        let listens = ListenBrainzAPI::get_user_username_listens_full()
            .client(&lbclient)
            .username(username)
            .maybe_start(start)
            .call()
            .await
            .context(LBApiRequestSnafu)?;

        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        // Remove the old listens
        Listen::delete_listen_range(
            &mut trans,
            start.unwrap_or(0) as i64,
            Utc::now().timestamp(),
            username,
        )
        .await?;

        save_listens(listens, &mut trans).await?;

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }
}

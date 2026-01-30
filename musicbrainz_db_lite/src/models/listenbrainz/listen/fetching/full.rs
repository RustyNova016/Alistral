use chrono::Utc;
use listenbrainz_rs::ListenBrainzAPIEnpoints;
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
    /// Fetch and save all the user listens
    pub async fn fetch_and_insert_full(
        client: &DBClient,
        username: &str,
    ) -> Result<(), ListenFetchingError> {
        // Get the new listens
        let listens = ListenBrainzAPIEnpoints::get_user_username_listens_full()
            .client(&client.listenbrainz_client)
            .username(username)
            .call()
            .await
            .context(LBApiRequestSnafu)?;

        let conn = &mut *client.database.get_conn().await.context(ConnectionSnafu)?;
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        // Remove the old listens
        Listen::delete_listen_range(&mut trans, 0, Utc::now().timestamp(), username).await?;

        save_listens(listens, &mut trans).await?;

        trans.commit().await.context(SqlxSnafu)?;

        Ok(())
    }
}

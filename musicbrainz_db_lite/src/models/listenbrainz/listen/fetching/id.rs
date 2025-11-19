use listenbrainz_rs::api::ListenBrainzAPI;
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
    /// Fetch and save a listen by its index (listened_at, user, msid)
    pub async fn fetch_and_insert_by_index(
        client: &DBClient,
        listened_at: i64,
        username: &str,
        msid: &str,
    ) -> Result<Option<Listen>, ListenFetchingError> {
        let start = listened_at - 1;
        let end = listened_at + 1;

        // Get the new listens
        let listens = ListenBrainzAPI::get_user_username_listens_full()
            .client(&client.listenbrainz_client)
            .username(username)
            .start(start as u64)
            .end(end as u64)
            .call()
            .await
            .context(LBApiRequestSnafu)?;

        let conn = &mut *client.database.get_conn().await.context(ConnectionSnafu)?;
        let mut trans = conn.begin().await.context(SqlxSnafu)?;

        // Remove the old listens
        Listen::delete_listen_range(&mut trans, start, end, username).await?;

        let listens = save_listens(listens, &mut trans).await?;

        trans.commit().await.context(SqlxSnafu)?;

        Ok(listens.into_iter().find(|listen| {
            listen.listened_at == listened_at
                && listen.user == username
                && listen.recording_msid == msid
        }))
    }
}

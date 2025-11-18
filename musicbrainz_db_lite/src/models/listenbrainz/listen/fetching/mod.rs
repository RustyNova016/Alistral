use std::sync::Arc;

use chrono::Utc;
use listenbrainz_rs::api::ListenBrainzAPI;
use listenbrainz_rs::client::ListenBrainzClient;
use listenbrainz_rs::client::api_request::error::ApiRequestError;
use sequelles::databases::sqlite::database::GetConnectionError;
use snafu::ResultExt;
use sqlx::Acquire;

use crate::DBClient;
use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::listenbrainz::listen::Listen;

impl Listen {
    /// Fetch and save all the user listens
    pub async fn fetch_and_save_full(
        &self,
        client: Arc<DBClient>,
        username: &str,
    ) -> Result<(), ListenFetchingError> {
        //TODO: Add the client to the main client
        let lbclient = ListenBrainzClient::new();

        // Get the new listens
        let listens = ListenBrainzAPI::get_user_username_listens_full()
            .client(&lbclient)
            .username(username)
            .call()
            .await
            .context(LBApiRequestSnafu)?;

        let conn = &mut *client.database.get_conn().await.context(ConnectionSnafu)?;
        let mut trans = conn
            .begin()
            .await
            .context(SqlxSnafu)
            .context(InnerSqlxSnafu)?;

        // Remove the old listens
        Listen::delete_listen_range(&mut trans, 0, Utc::now().timestamp(), username)
            .await
            .context(InnerSqlxSnafu)?;

        for listen in listens {
            Listen::insert_user_listen_listen(&mut trans, listen)
                .await
                .context(InnerSqlxSnafu)?;
        }

        trans
            .commit()
            .await
            .context(SqlxSnafu)
            .context(InnerSqlxSnafu)?;

        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum ListenFetchingError {
    #[snafu(display("Couldn't fetch the listens"))]
    LBApiRequestError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: ApiRequestError,
    },

    ConnectionError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: GetConnectionError,
    },

    InnerSqlxError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: SqlxError,
    },
}

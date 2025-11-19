use chrono::Utc;
use listenbrainz_rs::api::ListenBrainzAPI;
use listenbrainz_rs::api::user::username::listens::UserListensListen;
use listenbrainz_rs::client::api_request::error::ApiRequestError;
use sequelles::databases::sqlite::database::GetConnectionError;
use snafu::ResultExt;
use sqlx::Acquire;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::DBClient;
use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::listenbrainz::listen::Listen;

pub mod id;
pub mod incremental;

impl Listen {
    /// Fetch and save all the user listens
    pub async fn fetch_and_insert_full(
        client: &DBClient,
        username: &str,
    ) -> Result<(), ListenFetchingError> {
        // Get the new listens
        let listens = ListenBrainzAPI::get_user_username_listens_full()
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

#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
async fn save_listens(
    listens: Vec<UserListensListen>,
    trans: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<Vec<Listen>, ListenFetchingError> {
    let mut saved_listens = Vec::with_capacity(listens.len());
    pg_counted!(listens.len(), "Saving Listens");

    for listen in listens {
        saved_listens.push(Listen::insert_user_listen_listen(trans, listen).await?);
        pg_inc!();
    }

    Ok(saved_listens)
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

    #[snafu(transparent)]
    SqlxError { source: SqlxError },
}

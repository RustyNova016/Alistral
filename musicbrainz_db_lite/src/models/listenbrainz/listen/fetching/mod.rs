use listenbrainz_rs::api::user::username::listens::UserListensListen;
use listenbrainz_rs::api::user::username::listens_reader::ListenFullFetchError;
use sequelles::databases::sqlite::database::GetConnectionError;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::DBClient;
use crate::error::sqlx_error::SqlxError;
use crate::models::listenbrainz::listen::Listen;

pub mod full;
pub mod id;
pub mod incremental;

impl Listen {
    pub async fn fetch_and_insert(
        client: &DBClient,
        username: &str,
        incremental: bool,
    ) -> Result<(), ListenFetchingError> {
        if incremental {
            Self::fetch_and_insert_incremental(client, username).await
        } else {
            Self::fetch_and_insert_full(client, username).await
        }
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
        source: ListenFullFetchError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ConnectionError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: GetConnectionError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(transparent)]
    SqlxError {
        source: SqlxError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

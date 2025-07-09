use musicbrainz_db_lite::database::pool::DBLitePoolError;
use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] musicbrainz_db_lite::Error),

    #[error(transparent)]
    DBConnectionError(#[from] DBLitePoolError),

    #[error(transparent)]
    DBRawConnectionError(#[from] RawPoolError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

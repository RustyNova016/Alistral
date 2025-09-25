use musicbrainz_db_lite::database::conn_pool::DBLitePoolError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] musicbrainz_db_lite::Error),

    #[error(transparent)]
    DBConnectionError(#[from] DBLitePoolError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

use musicbrainz_db_lite::sequelle::databases::sqlite::database::GetConnectionError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] musicbrainz_db_lite::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    GetConnectionError(#[from] GetConnectionError),
}

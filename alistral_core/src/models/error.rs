use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;
use thiserror::Error;

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum Error {
    #[error(transparent)]
    MusicbrainzDBLiteError(#[from] musicbrainz_db_lite::Error),

    #[error("Tried to get user {0} but couldn't be found")]
    MissingUserError(String),

    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),

    #[error(transparent)]
    DatabasePoolError(#[from] RawPoolError),
}

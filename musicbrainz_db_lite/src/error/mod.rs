use deadpool::managed::PoolError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    //#[error(transparent)]
    //ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    MusicbrainzError(#[from] musicbrainz_rs_nova::Error),

    #[error(transparent)]
    ListenbrainzError(#[from] listenbrainz::Error),

    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("The MBID {0} wasn't found in Musicbrainz, but found in the local database. Hint: The upstream MBID might have been deleted")]
    UnknownUpstream(String),

    // Temporary errors
    #[error("Tried to insert a relation that is not yet implemented")]
    RelationNotImplemented, //TODO: Remove when all relations are implemented

    // Fetching Errors
    #[error("Fetching a listen from Listenbrainz resulted in timeout")]
    ListenFetchingTimeout,

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DeadpoolError(#[from] PoolError<sqlx::Error>),
}

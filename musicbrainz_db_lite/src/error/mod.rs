use deadpool::managed::PoolError;
use futures::channel::mpsc::SendError;
use thiserror::Error;

use crate::api::listenbrainz::listen::fetching::query::ListenFetchQueryError;

#[derive(Error, Debug)]
pub enum Error {
    //#[error(transparent)]
    //ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    MusicbrainzError(#[from] musicbrainz_rs::Error),

    #[error(transparent)]
    ListenbrainzError(#[from] listenbrainz::Error),

    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error(
        "The MBID {0} wasn't found in Musicbrainz, but found in the local database. Hint: The upstream MBID might have been deleted"
    )]
    NotFoundInUpstream(String),

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

    #[error(transparent)]
    ListenFetchQueryError(#[from] ListenFetchQueryError),

    #[error("Listenbrainz returned an error while fetching listens: {0}")]
    ListenFetchingError(String),

    #[error(transparent)]
    SendError(#[from] SendError),
}

use deadpool::managed::PoolError;
use futures::channel::mpsc::SendError;
use sequelles::databases::sqlite::database::GetConnectionError;
use thiserror::Error;

use crate::api::listenbrainz::listen::fetching::query::ListenFetchQueryError;
use crate::models::errors::sqlx_error::SqlxError;

#[derive(Error, Debug)]
pub enum Error {
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

    #[error(transparent)]
    SqlxError2(#[from] SqlxError),

    #[error(transparent)]
    GetConnectionError(#[from] GetConnectionError),

    #[error(transparent)]
    MusicbrainzGetRequestError(#[from] musicbrainz_rs::GetRequestError),
}

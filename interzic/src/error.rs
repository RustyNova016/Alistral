use std::io;
use thiserror::Error;

use crate::models::services::youtube::error::YoutubeError;

#[derive(Error, Debug)]
pub enum Error {
    // --- Config Errors ---
    #[error("An error occured when trying to load the provided secret file.")]
    SecretFileLoadError(io::Error),

    #[error(transparent)]
    MusicbrainzError(#[from] musicbrainz_rs::Error),

    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),

    #[error(transparent)]
    ListenBrainzError(#[from] listenbrainz::Error),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("The client is missing: {0}")]
    ClientBuildingError(String),

    #[error("The recording doesn't have an MBID set, but the function requires one")]
    MissingRequiredMBIDError(),

    #[error("This action require a listenbrainz client, but it wasn't set up in the main client")]
    MissingListenbrainzClient(),

    #[error(
        "This action require a musicbrainz_db_lite client, but it wasn't set up in the main client"
    )]
    MissingMusicbrainzDbLiteClient,

    #[error(
        "This action require a musicbrainz_rs client, but it wasn't set up in the main client"
    )]
    MissingMusicbrainzClient,

    // --- Service Errors ---
    #[error(transparent)]
    YoutubeError(#[from] YoutubeError),

    #[error(transparent)]
    MusicbrainzDBLiteError(#[from] musicbrainz_db_lite::Error),
}

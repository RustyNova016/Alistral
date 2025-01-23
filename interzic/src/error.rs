use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // --- Config Errors ---
    #[error("An error occured when trying to load the provided secret file.")]
    SecretFileLoadError(io::Error),

    #[error(transparent)]
    ApiError(#[from] google_youtube3::common::Error),

    #[error(transparent)]
    MusicbrainzError(#[from] musicbrainz_rs::Error),

    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("The client is missing: {0}")]
    ClientBuildingError(String),

    #[error("The recording doesn't have an MBID set, but the function requires one")]
    MissingRequiredMBIDError(),

    #[error("This action require a youtube client, but it wasn't set up in the main client")]
    MissingYoutubeClient(),
    
    #[error(transparent)]
    MusicbrainzDBLiteError(#[from] musicbrainz_db_lite::Error),
}

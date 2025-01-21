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
}

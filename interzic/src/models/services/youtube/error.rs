use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeError {
    #[error(transparent)]
    ApiError(#[from] google_youtube3::common::Error),

    #[error("This action require a youtube client, but it wasn't set up in the main client")]
    MissingYoutubeClient(),

    //#[error("An error happened when creating a playlist")]
    #[error(transparent)]
    PlaylistCreateError(google_youtube3::common::Error),

    //#[error("An error happened when adding a video to a playlist: {0}")]
    #[error(transparent)]
    PlaylistInsertError(google_youtube3::common::Error),

    //#[error("Couldn't search the recording")]
    #[error(transparent)]
    RecordingSearchError(google_youtube3::common::Error),
}

#[derive(Debug, Deserialize)]
pub struct BadRequestError {
    pub error: BadRequestErrorError,
}

#[derive(Debug, Deserialize)]
pub struct BadRequestErrorError {
    pub code: i64,
    pub errors: Vec<BadRequestErrorErrorItem>,
}

#[derive(Debug, Deserialize)]
pub struct BadRequestErrorErrorItem {
    pub domain: String,
    pub reason: String,
}

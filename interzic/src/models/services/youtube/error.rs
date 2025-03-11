use serde::Deserialize;
use thiserror::Error;

use crate::models::messy_recording::MessyRecording;

/// Errors for the actions of interzic for youtube
#[derive(Error, Debug)]
pub enum InterzicYoutubeError {
    #[error(transparent)]
    ApiError(#[from] YoutubeError),

    #[error("This action require a youtube client, but it wasn't set up in the main client")]
    MissingYoutubeClient(),

    //#[error("An error happened when creating a playlist")]
    #[error(transparent)]
    PlaylistCreateError(YoutubeError),

    //#[error("An error happened when adding a video to a playlist: {0}")]
    #[error(transparent)]
    PlaylistInsertError(YoutubeError),

    //#[error("Couldn't search the recording")]
    #[error(transparent)]
    RecordingSearchError(YoutubeError),

    #[error("Couldn't find the recording \"{0}\" on youtube")]
    RecordingSearchNotFoundError(MessyRecording),
}

impl InterzicYoutubeError {
    pub fn as_youtube_error(&self) -> Option<&YoutubeError> {
        match self {
            Self::ApiError(val) => Some(val),
            Self::PlaylistCreateError(val) => Some(val),
            Self::PlaylistInsertError(val) => Some(val),
            Self::RecordingSearchError(val) => Some(val),
            Self::MissingYoutubeClient() => None,
            Self::RecordingSearchNotFoundError(_) => None,
        }
    }
}

impl From<google_youtube3::common::Error> for InterzicYoutubeError {
    fn from(value: google_youtube3::common::Error) -> Self {
        Self::ApiError(value.into())
    }
}

/// Errors from youtube
#[derive(Error, Debug)]
pub enum YoutubeError {
    #[error(transparent)]
    ApiError(google_youtube3::common::Error),

    #[error(transparent)]
    QuotaExceededError(google_youtube3::common::Error),

    #[error(transparent)]
    BadServiceError(google_youtube3::common::Error),

    /// When a trying to add an unknown video to a playlist
    #[error("Tryed to add an unknown video to the playlist. Id: {1}")]
    Add404VideoError(google_youtube3::common::Error, String),

    #[error("The api didn't return a playlist ID")]
    MissingPlaylistIDError,
}

impl YoutubeError {
    pub fn is_bad_service_error(&self) -> bool {
        matches!(self, Self::BadServiceError(_))
    }
}

impl From<google_youtube3::common::Error> for YoutubeError {
    fn from(value: google_youtube3::common::Error) -> Self {
        if is_quota_exceeded_error(&value) {
            Self::QuotaExceededError(value)
        } else if is_bad_service_error(&value) {
            Self::BadServiceError(value)
        } else {
            Self::ApiError(value)
        }
    }
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

pub(super) fn is_quota_exceeded_error(err: &google_youtube3::common::Error) -> bool {
    let google_youtube3::common::Error::BadRequest(err) = err else {
        return false;
    };

    let err: Result<BadRequestError, serde_json::Error> = serde_json::from_value(err.clone());

    let Ok(err) = err else {
        return false;
    };

    err.error.code == 403
        && err
            .error
            .errors
            .iter()
            .any(|err| err.reason == "quotaExceeded")
}

fn is_bad_service_error(err: &google_youtube3::common::Error) -> bool {
    let google_youtube3::common::Error::BadRequest(err) = err else {
        return false;
    };

    let err: Result<BadRequestError, serde_json::Error> = serde_json::from_value(err.clone());

    let Ok(err) = err else {
        return false;
    };

    err.error.code == 409
        && err
            .error
            .errors
            .iter()
            .any(|err| err.reason == "SERVICE_UNAVAILABLE")
}

pub(super) fn is_add_404_video_error(err: &google_youtube3::common::Error) -> bool {
    let google_youtube3::common::Error::BadRequest(err) = err else {
        return false;
    };

    let err: Result<BadRequestError, serde_json::Error> = serde_json::from_value(err.clone());

    let Ok(err) = err else {
        return false;
    };

    err.error.code == 404
        && err
            .error
            .errors
            .iter()
            .any(|err| err.reason == "videoNotFound")
}

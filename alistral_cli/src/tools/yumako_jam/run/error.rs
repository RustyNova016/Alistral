use std::path::PathBuf;

use yumako_jams::models::radio_file::error::RadioFileError;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum YumakoRunCommandError {
    /// Error while getting the recording statistics
    RadioNotFound {
        path: PathBuf,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't read the radio file
    RadioFileReadError {
        source: std::io::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't parse the radio file
    RadioFileParseError {
        source: RadioFileError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't parse the inputs file
    RadioInputsParseError {
        source: json5::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't compile the radio
    RadioCompilationError {
        source: yumako_jams::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't convert the recordings for a playlist
    PlaylistConvertError {
        source: interzic::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for YumakoRunCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RadioNotFound { path, .. } => Some(FriendlyPanic {
                title: "Radio not found".to_string(),
                body: format!(
                    "Couldn't find the radio at `{}` (or .json5). Make sure it exists and there is no typos",
                    path.display()
                ),
            }),

            Self::RadioFileParseError { source, .. } => Some(FriendlyPanic {
                title: "Radio not parsable".to_string(),
                body: format!(
                    "The radio file couldn't be parsed. Make sure it is correct json/json5 and that it matches the radio format\n\n-----\n\n{}",
                    source
                ),
            }),

            Self::RadioInputsParseError { source, .. } => Some(FriendlyPanic {
                title: "Inputs not parsable".to_string(),
                body: format!(
                    "The inputs couldn't be parsed. Make sure it is correct json/json5\n\n-----\n\n{}",
                    source
                ),
            }),

            Self::RadioCompilationError { source, .. } => Some(FriendlyPanic {
                title: "Radio Compilation Error".to_string(),
                body: format!(
                    "There was an error during radio compilation\n\n-----\n\n{}",
                    source
                ),
            }),

            Self::RadioFileReadError { .. } => None,
            Self::PlaylistConvertError { .. } => None,
        }
    }
}

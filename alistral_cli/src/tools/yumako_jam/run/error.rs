use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::utils::yumako_jams::get_radio::YumakoGetRadioError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum YumakoRunCommandError {
    /// Error while getting the recording statistics
    YumakoGetRadioError {
        source: YumakoGetRadioError,

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
            Self::YumakoGetRadioError { source, .. } => source.get_friendly_error(),

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
            
            Self::PlaylistConvertError { .. } => None,
        }
    }
}

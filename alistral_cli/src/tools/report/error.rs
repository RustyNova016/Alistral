use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::report::sections::error::SectionsError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum ReportCommandError {
    /// Error while getting the recording statistics
    ParsingPeriodError {
        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display("Unknown preset: {preset}"))]
    UnknownPreset {
        preset: String,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while processing a section
    SectionsError {
        #[snafu(source(from(SectionsError, Box::new)))]
        source: Box<SectionsError>,
        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Couldn't parse the time interval
    IntervalError {
        interval: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for ReportCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::ParsingPeriodError { .. } => Some(FriendlyPanic {
                title: "Couldn't parse the input period".to_string(),
                body: "You may have a typo".to_string(),
            }),

            Self::UnknownPreset { preset, .. } => Some(FriendlyPanic {
                title: "Unknown preset".to_string(),
                body: format!("Preset {} is not in the configuration file", preset),
            }),

            Self::IntervalError { interval, .. } => Some(FriendlyPanic {
                title: "Unknown interval".to_string(),
                body: format!("Interval `{}` is not a proper interval string", interval),
            }),

            Self::SectionsError { source, .. } => source.get_friendly_error(),
        }
    }
}

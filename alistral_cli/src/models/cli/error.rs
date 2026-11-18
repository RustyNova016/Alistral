use snafu::Snafu;
#[cfg(feature = "interzic")]
use tuillez::fatal_error::FatalError;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::bump::BumpCommandError;
use crate::tools::cache::CacheCommandError;
use crate::tools::daily::error::DailyCommandError;
#[cfg(feature = "yumako_jams")]
use crate::tools::yumako_jam::YumakoCommandError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
#[expect(
    clippy::enum_variant_names,
    reason = "Error passthough keeps its error name"
)]
pub enum CLIError {
    /// Error while running the `bump` command
    BumpCommandError {
        source: BumpCommandError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `cache` command
    CacheCommandError {
        source: CacheCommandError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `config` command
    ConfigCommandError {
        #[snafu(source(from(crate::Error, Box::new)))]
        source: Box<crate::Error>,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `daily` command
    DailyCommandError {
        source: DailyCommandError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `interzic` command
    #[cfg(feature = "interzic")]
    InterzicCommandError {
        source: FatalError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `playlist` command
    #[cfg(feature = "interzic")]
    PlaylistCommandError {
        source: FatalError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `radio` command
    #[cfg(feature = "radio")]
    RadioCommandError {
        #[snafu(source(from(crate::Error, Box::new)))]
        source: Box<crate::Error>,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while running the `yumako-jams` command
    #[cfg(feature = "yumako_jams")]
    YumakoCommandError {
        source: YumakoCommandError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for CLIError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::BumpCommandError { source, .. } => source.get_friendly_error(),
            Self::CacheCommandError { source, .. } => source.get_friendly_error(),
            Self::ConfigCommandError { source, .. } => source.get_friendly_error(),
            Self::DailyCommandError { source, .. } => source.get_friendly_error(),
            #[cfg(feature = "interzic")]
            Self::InterzicCommandError { .. } => None,
            #[cfg(feature = "interzic")]
            Self::PlaylistCommandError { .. } => None,
            #[cfg(feature = "radio")]
            Self::RadioCommandError { source, .. } => source.get_friendly_error(),
            #[cfg(feature = "yumako_jams")]
            Self::YumakoCommandError { source, .. } => source.get_friendly_error(),
        }
    }
}

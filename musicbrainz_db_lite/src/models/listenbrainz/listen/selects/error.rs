use sequelles::databases::sqlite::database::GetConnectionError;

use crate::models::listenbrainz::listen::fetching::ListenFetchingError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
#[allow(clippy::large_enum_variant)]
pub enum ListenFetchGetError {
    #[snafu(display("Couldn't fetch the listens"))]
    ListenFetchingError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: ListenFetchingError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ConnectionError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: GetConnectionError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ListenSelectError {
        source: sqlx::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

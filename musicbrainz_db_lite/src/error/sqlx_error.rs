use snafu::Location;
use snafu::Snafu;

/// Wrapper around [sqlx::Error] with backtrace support
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)), display("An error occured in SQLx"))]
pub struct SqlxError {
    source: sqlx::Error,

    #[snafu(implicit)]
    location: Location,

    #[cfg(feature = "backtrace")]
    backtrace: snafu::Backtrace,
}

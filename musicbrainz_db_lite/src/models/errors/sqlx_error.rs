use snafu::Backtrace;
use snafu::Snafu;

/// Wrapper around [sqlx::Error] with backtrace support
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub struct SqlxError {
    source: sqlx::Error,
    backtrace: Backtrace,
}

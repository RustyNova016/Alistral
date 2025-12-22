#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum SubsonicServiceError {
    #[snafu(display("The subsonic server returned an error"))]
    SubsonicError {
        source: submarine::SubsonicError,

        #[snafu(implicit)]
        location: snafu::Location,

        // For non snafu sources
        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

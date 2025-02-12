use std::io;
use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;
use thiserror::Error;
use tuillez::extensions::chrono_exts::TimeError;
use tuillez::fatal_error::FatalError;

use crate::interface::errors::process_errors;

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum Error {
    #[error(transparent)]
    AlistralCore(#[from] alistral_core::Error),

    #[error(transparent)]
    InterzicError(#[from] interzic::Error),

    #[error(transparent)]
    TimeError(#[from] TimeError),

    // --- Config Errors ---
    #[error("An error occured when trying to load the configuration file.")]
    ConfigLoadError(io::Error),

    #[error("Couldn't load the configuration file. The configuration schema is incorrect")]
    ConfigLoadDeserializationError(serde_json::Error),

    #[error("Couldn't create the configuration file.")]
    ConfigFileCreationError(io::Error),

    #[error("Couldn't write the configuration file.")]
    ConfigFileWriteError(serde_json::Error),

    // --- Cache Errors ---
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),

    #[error(transparent)]
    MusicbrainzDBLiteError(#[from] musicbrainz_db_lite::Error),

    #[error("Tried to get user {0} but couldn't be found")]
    MissingUserError(String),

    #[error("Tried to open the database {0} but it couldn't be found")]
    MissingDatabaseFile(String),

    #[error("Filesystem error when accessing the cache")]
    DatabaseIoError(io::Error),

    // --- Fetching Errors ---
    #[error("Error with the request.")]
    RequestError(#[from] reqwest::Error),

    #[error("Couldn't decode the server's responce")]
    RequestDecodeError(reqwest::Error),

    #[error("Listenbrainz responded with an error")]
    ListenbrainzError(#[from] listenbrainz::Error),

    #[error("No user data is available for this playlist export target: {0}")]
    MissingPlaylistUserDataError(String),

    #[error(transparent)]
    FatalError(#[from] FatalError),

    #[error(transparent)]
    RawConnectionError(#[from] RawPoolError),
}

impl From<Error> for FatalError {
    fn from(value: Error) -> Self {
        match value {
            Error::FatalError(val) => val,
            _ => {
                let text = value.get_help();
                FatalError::new(value, text)
            }
        }
    }
}

impl Error {
    pub fn from_musicbrainz_rs_error(err: reqwest::Error) -> Self {
        if err.is_decode() {
            return Self::RequestDecodeError(err);
        }

        Self::RequestError(err)
    }

    pub fn expect_fatal(self, text: String) -> ! {
        let text = process_errors(&self).or(Some(text));

        println!("{}", FatalError::new(self, text));
        panic!()
    }

    pub fn get_help(&self) -> Option<String> {
        process_errors(self)
    }
}

#[extend::ext]
pub impl<T, E> Result<T, E>
where
    E: Into<crate::Error>,
{
    fn unwrap_fatal(self) -> T {
        match self {
            Ok(v) => v,
            Err(err) => inner(err, None),
        }
    }

    fn expect_fatal(self, text: &str) -> T {
        match self {
            Ok(v) => v,
            Err(err) => inner(err, Some(text.to_string())),
        }
    }
}

fn inner<T: Into<crate::Error>>(this: T, text: Option<String>) -> ! {
    let err: crate::Error = this.into();
    let text = process_errors(&err).or(text);
    FatalError::new(err, text).panic()
}

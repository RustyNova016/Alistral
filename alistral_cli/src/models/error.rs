use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;
use std::io;
use thiserror::Error;
use tuillez::extensions::chrono_exts::TimeError;
use tuillez::fatal_error::FatalError;

use crate::interface::errors::process_errors;

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum Error {
    // --- Config Errors ---
    #[error("An error occured when trying to load the configuration file.")]
    ConfigRead(io::Error),

    #[error("Couldn't load the configuration file. The configuration schema is incorrect")]
    ConfigDeserialization(serde_json::Error),

    #[error("Couldn't create the configuration file.")]
    ConfigFileCreation(io::Error),

    #[error("Couldn't write the configuration file.")]
    ConfigFileWrite(serde_json::Error),

    // --- Cache Errors ---
    #[error("Tried to get user {0} but couldn't be found")]
    MissingUser(String),

    #[error("Tried to open the database {0} but it couldn't be found")]
    MissingDatabaseFile(String),

    #[error("Filesystem error when accessing the cache")]
    DatabaseIo(io::Error),

    // --- Fetching Errors ---
    #[error("Error with the request.")]
    Reqwest(#[from] reqwest::Error),

    #[error("Couldn't decode the server's responce")]
    RequestDecode(reqwest::Error),

    #[error("Listenbrainz responded with an error")]
    Listenbrainz(#[from] listenbrainz::Error),

    #[error("No user data is available for this playlist export target: {0}")]
    MissingPlaylistUserData(String),

    #[allow(clippy::enum_variant_names)]
    #[error(transparent)]
    FatalError(#[from] FatalError),

    #[error("Couldn't parse the arguments")]
    YumakoArgumentParsingError(), //TODO: Impl error for lyn::Error

    #[error(
        "Couldn't read the data of a variable. \nVariable: {0}. \nProvided data: {1}\nError: {2}"
    )]
    YumakoArgumentDataDeserializingError(String, String, serde_json::Error),

    #[error(transparent)]
    RawConnection(#[from] RawPoolError),

    // ==================
    //  Error Reexports
    // ==================
    #[error(transparent)]
    AlistralCore(#[from] alistral_core::Error),

    #[cfg(feature = "interzicf")]
    #[error(transparent)]
    Interzic(#[from] interzic::Error),

    #[allow(clippy::enum_variant_names)]
    #[error(transparent)]
    TimeError(#[from] TimeError),

    #[error(transparent)]
    SQLx(#[from] sqlx::Error),

    #[error(transparent)]
    MusicbrainzDBLite(#[from] musicbrainz_db_lite::Error),
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
            return Self::RequestDecode(err);
        }

        Self::Reqwest(err)
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

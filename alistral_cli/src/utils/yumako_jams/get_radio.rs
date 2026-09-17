use std::fs::File;
use std::io::Read as _;
use std::path::PathBuf;

use snafu::ResultExt as _;
use yumako_jams::models::radio_file::error::RadioFileError;
use yumako_jams::models::radio_file::radio::Radio;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::utils::constants::paths::YUMAKO_DIR;

pub fn get_radio(name: &str) -> Result<Radio, YumakoGetRadioError> {
    let path = get_radio_path(name)?;

    let mut file = File::open(&path).context(RadioFileReadSnafu)?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .context(RadioFileReadSnafu)?;

    Radio::from_file_content(&content).context(RadioFileParseSnafu)
}

fn get_radio_path(name: &str) -> Result<PathBuf, YumakoGetRadioError> {
    let path = YUMAKO_DIR.join(format!("{name}.json5"));
    if path.exists() {
        return Ok(path);
    }

    let path = YUMAKO_DIR.join(format!("{name}.json"));
    if path.exists() {
        return Ok(path);
    }

    RadioNotFoundSnafu { path }.fail()
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum YumakoGetRadioError {
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
}

impl GetFriendlyError for YumakoGetRadioError {
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

            Self::RadioFileReadError { .. } => None,
        }
    }
}

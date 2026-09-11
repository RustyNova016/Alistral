use std::path::PathBuf;

use futures::io;
use snafu::Snafu;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::FriendlyPanicSnafu;
use crate::interface::errors::friendly_error::GetFriendlyError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum YumakoCommandError {
    #[snafu(display("Cannot read the input file"))]
    InputFileError {
        source: io::Error,
        path: PathBuf,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for YumakoCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::InputFileError { path, .. } => Some(
                FriendlyPanicSnafu {
                    title: "Cannot read an input file",
                    body: format!(
                        "Couldn't read the input variable file at `{}`. Is it a valid path?",
                        path.display()
                    ),
                }
                .build(),
            ),
        }
    }
}

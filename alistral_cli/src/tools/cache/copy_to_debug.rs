use std::fs;

use clap::Parser;
use snafu::ResultExt;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::cache::delete_database;

/// Copy the release database to the debug one.
///
/// ⚠️ This wipes the debug database.
#[derive(Parser, Debug, Clone)]
pub struct CacheCopyToDebugCommand;

impl CacheCopyToDebugCommand {
    pub fn run(&self) -> Result<(), CacheCopyToDebugCommandError> {
        delete_database(&DEBUG_DB_LOCATION).context(DeleteDatabaseSnafu)?;

        fs::copy(&*RELEASE_DB_LOCATION, &*DEBUG_DB_LOCATION).context(CopySnafu)?;

        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum CacheCopyToDebugCommandError {
    DeleteDatabase { source: std::io::Error },
    CopyError { source: std::io::Error },
}

impl GetFriendlyError for CacheCopyToDebugCommandError {
    fn get_friendly_error(
        &self,
    ) -> Option<crate::interface::errors::friendly_error::FriendlyPanic> {
        match self {
            Self::DeleteDatabase { source: _ } => None,
            Self::CopyError { source: _ } => None,
        }
    }
}

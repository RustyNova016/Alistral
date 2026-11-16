use std::fs;

use clap::Parser;
use snafu::ResultExt as _;

use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::directories::DIRECTORIES;
use crate::tools::cache::delete_database;

/// Copy the release database to the debug one.
///
/// ⚠️ This wipes the debug database.
#[derive(Parser, Debug, Clone)]
pub struct CacheCopyToDebugCommand;

impl CacheCopyToDebugCommand {
    pub fn run(&self) -> Result<(), CacheCopyToDebugCommandError> {
        delete_database(&DIRECTORIES.musicbrainz_debug_cache_path())
            .context(DeleteDatabaseSnafu)?;

        fs::copy(
            DIRECTORIES.musicbrainz_release_cache_path(),
            &DIRECTORIES.musicbrainz_debug_cache_path(),
        )
        .context(CopySnafu)?;

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
            Self::DeleteDatabase { .. } => None,
            Self::CopyError { .. } => None,
        }
    }
}

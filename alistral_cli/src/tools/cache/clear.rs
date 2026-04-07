use clap::Parser;
use snafu::ResultExt;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::cache::delete_database;

/// Wipe the cache's data
///
/// This is useful if you need disk space, or need to manually rebuild in case of corruption
///
/// Most times you only need the `--main` argument. If you need to delete the debug database, use `--debug`
#[derive(Parser, Debug, Clone)]
pub struct CacheClearCommand {
    #[arg(short, long)]
    main: bool,

    #[arg(short, long)]
    debug: bool,
}

impl CacheClearCommand {
    pub fn run(&self) -> Result<(), CacheClearCommandError> {
        if self.main {
            delete_database(&RELEASE_DB_LOCATION).context(DeleteDatabaseSnafu)?;
        }

        if self.debug {
            delete_database(&DEBUG_DB_LOCATION).context(DeleteDebugDatabaseSnafu)?;
        }

        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum CacheClearCommandError {
    DeleteDatabase { source: std::io::Error },
    DeleteDebugDatabase { source: std::io::Error },
}

impl GetFriendlyError for CacheClearCommandError {
    fn get_friendly_error(
        &self,
    ) -> Option<crate::interface::errors::friendly_error::FriendlyPanic> {
        match self {
            Self::DeleteDatabase { source: _ } => None,
            Self::DeleteDebugDatabase { source: _ } => None,
        }
    }
}

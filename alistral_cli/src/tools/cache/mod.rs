use std::fs::remove_file;
use std::io;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use snafu::ResultExt;

use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::tools::cache::clear::CacheClearCommand;
use crate::tools::cache::clear::CacheClearCommandError;
use crate::tools::cache::copy_to_debug::CacheCopyToDebugCommand;
use crate::tools::cache::copy_to_debug::CacheCopyToDebugCommandError;

pub mod clear;
pub mod copy_to_debug;

pub fn delete_database(path: &Path) -> Result<(), io::Error> {
    delete_or_not_found(path)?;
    delete_or_not_found(format!("{}-wal", path.to_string_lossy()))?;
    delete_or_not_found(format!("{}-shm", path.to_string_lossy()))?;

    Ok(())
}

fn delete_or_not_found<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    match remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(());
            }

            Err(err)
        }
    }
}

/// Commands that interface with the cache of the app
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CacheSubcommands {
    Clear(CacheClearCommand),
    CopyToDebug(CacheCopyToDebugCommand),
}

impl CacheCommand {
    pub async fn run(&self) -> Result<(), CacheCommandError> {
        match &self.command {
            CacheSubcommands::Clear(val) => val.run().context(CacheClearCommandSnafu),
            CacheSubcommands::CopyToDebug(val) => val.run().context(CacheCopyToDebugCommandSnafu),
        }
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum CacheCommandError {
    CacheClearCommand {
        source: CacheClearCommandError,
    },
    CacheCopyToDebugCommand {
        source: CacheCopyToDebugCommandError,
    },
}

impl GetFriendlyError for CacheCommandError {
    fn get_friendly_error(
        &self,
    ) -> Option<crate::interface::errors::friendly_error::FriendlyPanic> {
        match self {
            Self::CacheClearCommand { source } => source.get_friendly_error(),
            Self::CacheCopyToDebugCommand { source } => source.get_friendly_error(),
        }
    }
}

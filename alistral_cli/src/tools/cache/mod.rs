use std::fs;
use std::fs::remove_file;
use std::io;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::tools::cache::clear::CacheClearCommand;
use crate::tools::cache::copy_to_debug::CacheCopyToDebugCommand;

pub mod clear;
pub mod copy_to_debug;
pub mod refresh_data;

pub fn delete_database(path: &Path) -> Result<(), crate::Error> {
    delete_or_not_found(path)?;
    delete_or_not_found(format!("{}-wal", path.to_string_lossy()))?;
    delete_or_not_found(format!("{}-shm", path.to_string_lossy()))?;

    Ok(())
}

fn delete_or_not_found<P: AsRef<Path>>(path: P) -> Result<(), crate::Error> {
    match remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(());
            }

            Err(crate::Error::DatabaseIo(err))
        }
    }
}

pub fn copy_to_debug() {
    delete_database(&DEBUG_DB_LOCATION).expect("Couldn't delete database");

    fs::copy(&*RELEASE_DB_LOCATION, &*DEBUG_DB_LOCATION).expect("Couldn't copy the database");
}

/// Commands that interface with the cache of the app
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CacheCommand {
    #[command(subcommand)]
    pub command: CacheSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum CacheSubcommands {
    Clear(CacheClearCommand),
    CopyToDebug(CacheCopyToDebugCommand),
}

impl CacheCommand {
    pub async fn run(&self) {
        match &self.command {
            CacheSubcommands::Clear(val) => val.run(),
            CacheSubcommands::CopyToDebug(val) => val.run(),
        }
    }
}

use std::fs;

use clap::Parser;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::tools::cache::delete_database;

/// Copy the release database to the debug one.
///
/// ⚠️ This wipes the debug database.
#[derive(Parser, Debug)]
pub struct CacheCopyToDebugCommand;

impl CacheCopyToDebugCommand {
    pub fn run(&self) {
        delete_database(&DEBUG_DB_LOCATION).expect("Couldn't delete database");

        fs::copy(&*RELEASE_DB_LOCATION, &*DEBUG_DB_LOCATION).expect("Couldn't copy the database");
    }
}

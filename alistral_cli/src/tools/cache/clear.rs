use clap::Parser;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::tools::cache::delete_database;

/// Wipe the cache's data
///
/// This is useful if you need disk space, or need to manually rebuild in case of corruption
///
/// Most times you only need the `--main` argument. If you need to delete the debug database, use `--debug`
#[derive(Parser, Debug)]
pub struct CacheClearCommand {
    #[arg(short, long)]
    main: bool,

    #[arg(short, long)]
    debug: bool,
}

impl CacheClearCommand {
    pub fn run(&self) {
        if self.main {
            delete_database(&RELEASE_DB_LOCATION).expect("Couldn't delete database");
        }

        if self.debug {
            delete_database(&DEBUG_DB_LOCATION).expect("Couldn't delete database");
        }
    }
}

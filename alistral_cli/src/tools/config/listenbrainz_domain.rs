use clap::Parser;

use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use crate::models::config::Config;
use crate::models::config::config_trait::ConfigFile;
use crate::tools::cache::delete_database;

/// Wipe the cache's data
///
/// This is useful if you need disk space, or need to manually rebuild in case of corruption
///
/// Most times you only need the `--main` argument. If you need to delete the debug database, use `--debug`
#[derive(Parser, Debug, Clone)]
pub struct ConfigListenbrainzDomainCommand {
    domain: String
}

impl CacheClearCommand {
    pub fn run(&self) {
        let conf = Config::load_unguarded().expect("Couldn't load config");
        conf.listenbrainz_url
    }
}

use std::sync::LazyLock;

use listenbrainz::raw::Client;

use crate::models::config::Config;

pub mod fresh_releases;
pub mod global_listen_counts;

pub static LISTENBRAINZ_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let config = Config::load_or_panic();
    let config = config.read_or_panic();
    Client::new_with_url(&config.listenbrainz_url)
});

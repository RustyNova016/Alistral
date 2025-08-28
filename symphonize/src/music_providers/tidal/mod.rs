use std::sync::LazyLock;

use musicbrainz_db_lite::Url;
use url::Host;

pub static TIDAL_HOST: LazyLock<Host<&str>> = LazyLock::new(|| Host::Domain("tidal.com"));

pub fn is_tidal_url(url: &Url) -> bool {
    url.match_host(&TIDAL_HOST)
}

pub fn get_tidal_artist_id_from_url(url: &str) -> Option<&str> {
    url.split("/").last()
}

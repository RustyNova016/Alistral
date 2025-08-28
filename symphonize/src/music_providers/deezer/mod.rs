use std::sync::LazyLock;

use musicbrainz_db_lite::Url;
use url::Host;

pub static DEEZER_HOST: LazyLock<Host<&str>> = LazyLock::new(|| Host::Domain("www.deezer.com"));

pub fn is_deezer_url(url: &Url) -> bool {
    url.match_host(&DEEZER_HOST)
}

pub fn get_deezer_artist_id_from_url(url: &str) -> Option<&str> {
    url.split("/").last()
}

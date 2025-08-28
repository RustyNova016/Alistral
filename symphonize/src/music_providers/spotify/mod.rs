use std::sync::LazyLock;

use musicbrainz_db_lite::Url;
use url::Host;

pub static SPOTIFY_HOST: LazyLock<Host<&str>> = LazyLock::new(|| Host::Domain("open.spotify.com"));

pub fn is_spotify_url(url: &Url) -> bool {
    url.match_host(&SPOTIFY_HOST)
}

pub fn get_spotify_artist_id_from_url(url: &str) -> Option<&str> {
    url.split("/").last()
}

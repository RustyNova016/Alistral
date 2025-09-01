use std::sync::LazyLock;

use musicbrainz_db_lite::Url;
use regex::Regex;
use url::Host;

pub static BANDCAMP_HOST: LazyLock<Host<&str>> = LazyLock::new(|| Host::Domain("bandcamp.com"));
static BANDCAMP_URL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?:\/\/([a-z0-9]+)\.bandcamp\.com").unwrap());

pub fn is_bandcamp_url(url: &Url) -> bool {
    BANDCAMP_URL_REGEX.is_match(&url.ressource)
}

pub fn get_bandcamp_artist_id_from_url(url: &str) -> Option<&str> {
    let groups = BANDCAMP_URL_REGEX.captures(url)?;

    return groups.get(1).map(|matc| matc.as_str());
}

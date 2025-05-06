use std::sync::LazyLock;

use regex::Regex;

/// Regex to determine if an url is compatible with harmony
pub static HARMONY_SEED_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)https?:\/\/(open\.spotify\.com|music\.apple\.com|www\.deezer\.com|tidal\.com|www\.beatport\.com)")
        .unwrap()
});

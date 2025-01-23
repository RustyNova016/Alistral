use std::sync::LazyLock;

use regex::Regex;

pub(crate) static YOUTUBE_URL_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"((youtu.be/)|(v/)|(/u/\w/)|(embed/)|(watch\?))\??v?=?([^#&?]*)").unwrap()
});

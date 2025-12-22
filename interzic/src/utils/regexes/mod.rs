#[cfg(any(feature = "youtube"))]
use std::sync::LazyLock;

#[cfg(any(feature = "youtube"))]
use regex::Regex;

#[cfg(any(feature = "youtube"))]
pub(crate) static YOUTUBE_URL_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"((youtu.be/)|(v/)|(/u/\w/)|(embed/)|(watch\?))\??v?=?([^#&?]*)").unwrap()
});

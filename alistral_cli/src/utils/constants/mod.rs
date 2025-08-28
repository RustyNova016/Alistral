#[cfg(feature = "interzicf")]
use std::path::PathBuf;
use std::sync::LazyLock;

use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;

use crate::ALISTRAL_CLIENT;
#[cfg(feature = "interzicf")]
use crate::utils::constants::paths::CONFIG_DIR;

pub mod paths;

#[cfg(feature = "musicbrainz")]
pub static MUSIBRAINZ_FMT: LazyLock<MusicbrainzFormater> = LazyLock::new(|| MusicbrainzFormater {
    artist_credits: true,
    listenbrainz_link: false,
    client: ALISTRAL_CLIENT.musicbrainz_db.clone(),
});

pub static LISTENBRAINZ_FMT: LazyLock<MusicbrainzFormater> =
    LazyLock::new(|| MusicbrainzFormater {
        artist_credits: true,
        listenbrainz_link: true,
        client: ALISTRAL_CLIENT.musicbrainz_db.clone(),
    });

#[cfg(feature = "interzicf")]
pub static YT_SECRET_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_credentials.json");

    path
});

#[cfg(feature = "interzicf")]
pub static TOKENCACHE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_tokens.json");

    path
});

#[cfg(feature = "interzicf")]
pub static INTERZIC_DB: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("interzic.db");

    path
});

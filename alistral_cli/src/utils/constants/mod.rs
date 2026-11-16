use std::sync::LazyLock;

use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;

use crate::ALISTRAL_CLIENT;

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

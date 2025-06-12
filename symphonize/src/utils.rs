use std::sync::LazyLock;

use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;

use crate::SymphonyzeClient;

pub fn formater(client: &SymphonyzeClient) -> MusicbrainzFormater {
    MusicbrainzFormater {
        artist_credits: true,
        listenbrainz_link: false,
        client: client.mb_database.clone(),
    }
}

static HARMONY_DOMAINS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "https://open.spotify.com",
        "https://www.deezer.com",
        "https://music.apple.com",
        "https://tidal.com",
        "https://www.beatport.com",
    ]
});

pub fn link_supported_by_harmony(link: &str) -> bool {
    HARMONY_DOMAINS
        .iter()
        .any(|domain| link.starts_with(domain))
}

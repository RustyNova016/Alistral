use core::num::NonZero;
use std::sync::LazyLock;

use governor::Quota;
use governor::RateLimiter;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::HasUrls as _;
use tracing::debug;
use tracing::error;
use url::Host;

use crate::SymphonyzeClient;
use crate::sambl::api_results::ApiError;
use crate::sambl::api_results::ApiResult;

pub mod api_results;

static RATE_LIMIT: LazyLock<
    RateLimiter<
        governor::state::NotKeyed,
        governor::state::InMemoryState,
        governor::clock::QuantaClock,
        governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    >,
> = LazyLock::new(|| RateLimiter::direct(Quota::per_minute(NonZero::new(6).unwrap())));

pub async fn sambl_get_artist_albums(
    spotify_id: &str,
    mbid: &str,
) -> Result<Option<ApiResult>, crate::Error> {
    let url = format!(
        "https://sambl.lioncat6.com/api/compareArtistAlbums?spotifyId={spotify_id}&mbid={mbid}&full=true"
    );

    RATE_LIMIT.until_ready().await;
    fetch_sambl(mbid, &url).await
}

// #[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
async fn fetch_sambl(mbid: &str, url: &str) -> Result<Option<ApiResult>, crate::Error> {
    debug!("Calling SAMBL with url: {url}");
    // pg_spinner!("Fetching SAMBL data...");

    let text = reqwest::get(url).await?.text().await?;

    if let Ok(res) = serde_json::from_str(&text) {
        return Ok(res);
    } else if let Ok(res) = serde_json::from_str::<ApiError>(&text) {
        error!("Error while fetching artist ({mbid}) SAMBL data: {res:#?}")
    }

    panic!("Couldn't parse SAMBL's response: {text:#?}");
}

pub async fn sambl_get_album_for_artist(
    client: &SymphonyzeClient,
    artist: &Artist,
) -> Result<Option<ApiResult>, crate::Error> {
    let host = Host::Domain("open.spotify.com");
    let Some(spot_url) = artist.has_url_with_host(&client.mb_database, &host).await? else {
        return Ok(None);
    };
    let spot_id = spot_url
        .ressource
        .split("/")
        .last()
        .expect("Malformed spotify url");

    sambl_get_artist_albums(spot_id, &artist.mbid).await
}

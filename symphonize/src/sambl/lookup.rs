use futures::FutureExt;
use futures::Stream;
use futures::TryStreamExt;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::HasUrls as _;
use musicbrainz_db_lite::Url;
use streamies::TryStreamies as _;

use crate::SymphonyzeClient;
use crate::music_providers::bandcamp::get_bandcamp_artist_id_from_url;
use crate::music_providers::bandcamp::is_bandcamp_url;
use crate::music_providers::deezer::get_deezer_artist_id_from_url;
use crate::music_providers::deezer::is_deezer_url;
use crate::music_providers::spotify::get_spotify_artist_id_from_url;
use crate::music_providers::spotify::is_spotify_url;
use crate::music_providers::tidal::get_tidal_artist_id_from_url;
use crate::music_providers::tidal::is_tidal_url;
use crate::sambl::RATE_LIMIT;
use crate::sambl::SamblProviders;
use crate::sambl::api_results::AlbumData;
use crate::sambl::api_results::ApiResult;
use crate::sambl::fetch_sambl;

pub fn sambl_lookup_stream(
    client: &SymphonyzeClient,
    artist: &Artist,
) -> impl Stream<Item = Result<AlbumData, crate::Error>> {
    artist
        .get_entity_urls(&client.mb_database)
        .into_stream()
        .map_err(crate::Error::from)
        .flatten_ok_iter()
        .try_filter_map(async |url| {
            let Some((host, id)) = get_sambl_host_id(&url) else {
                return Ok(None);
            };

            return sambl_get_artist_albums(id, host.to_namespace(), &artist.mbid).await;
        })
        .map_ok(|album| album.album_data)
        .flatten_ok_iter()
}

fn get_sambl_host_id(url: &Url) -> Option<(SamblProviders, &str)> {
    if is_spotify_url(url) {
        Some((
            SamblProviders::Spotify,
            get_spotify_artist_id_from_url(&url.ressource).unwrap(),
        ))
    } else if is_deezer_url(url) {
        Some((
            SamblProviders::Deezer,
            get_deezer_artist_id_from_url(&url.ressource).unwrap(),
        ))
    } else if is_tidal_url(url) {
        Some((
            SamblProviders::Tidal,
            get_tidal_artist_id_from_url(&url.ressource).unwrap(),
        ))
    } else if is_bandcamp_url(url) {
        Some((
            SamblProviders::Bandcamp,
            get_bandcamp_artist_id_from_url(&url.ressource).unwrap(),
        ))
    } else {
        None
    }
}

pub async fn sambl_get_artist_albums(
    provider_id: &str,
    provider: &str,
    mbid: &str,
) -> Result<Option<ApiResult>, crate::Error> {
    let url = format!(
        "https://sambl.lioncat6.com/api/compareArtistAlbums?provider_id={provider_id}&provider={provider}&mbid={mbid}&full=true"
    );

    RATE_LIMIT.until_ready().await;
    fetch_sambl(mbid, &url).await
}

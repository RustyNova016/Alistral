use core::future::ready;
use core::sync::atomic::Ordering;
use std::sync::Arc;

use futures::Stream;
use futures::StreamExt;
use futures::channel::mpsc::Sender;
use futures::channel::mpsc::channel;
use futures::pin_mut;
use futures::stream;
use itertools::Itertools;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Url;
use symphonize::clippy::lints::sambl::SamblLint;
use symphonize::clippy::lints::sambl::missing_samble_release::MissingSamblReleaseLint;
use symphonize::sambl::api_results::AlbumData;
use symphonize::sambl::sambl_get_album_for_artist;
use tracing::debug;
use tracing::info;

use crate::ALISTRAL_CLIENT;
use crate::tools::musicbrainz::clippy::PRINT_LOCK;
use crate::tools::musicbrainz::clippy::PROCESSED_COUNT;
use crate::tools::musicbrainz::clippy::REFETCH_LOCK;
use crate::tools::musicbrainz::clippy::print_lint;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

pub async fn samble_clippy_poller(stream: impl Stream<Item = ()>) {
    pin_mut!(stream);
    while let Some(_val) = stream.next().await {}
}

pub fn samble_clippy_stream(
    filter: &WhitelistBlacklist<String>,
) -> (Sender<Arc<MainEntity>>, impl Stream<Item = ()>) {
    let (entity_send, entity_stream) = channel::<Arc<MainEntity>>(10);

    let stream = entity_stream
        .filter_map(async |entity| match entity.as_ref() {
            MainEntity::Artist(artist) => Some(Arc::new(artist.to_owned())),
            _ => None,
        })
        // Get the album data
        .map(async |artist| {
            sambl_get_album_for_artist(&ALISTRAL_CLIENT.symphonize, &artist)
                .await
                .unwrap()
                .map(|data| (artist, data))
        })
        .buffer_unordered(8)
        .filter_map(ready)
        .flat_map(|(artist, data)| {
            stream::iter(data.album_data).map(move |album| (artist.clone(), album))
        })
        // Fetch the urls
        .chunks(100)
        .map(async |albums| {
            let urls = albums
                .iter()
                .map(|a| a.1.url.as_str())
                .collect_vec();
            Url::fetch_and_save_by_ressource_bulk_as_task(
                ALISTRAL_CLIENT.musicbrainz_db.clone(),
                urls,
            )
            .await
            .unwrap();
            stream::iter(albums)
        })
        .buffer_unordered(8)
        .flatten()
        // Process albums
        .map(|(artist, album)| process_album(artist, album, filter))
        .buffer_unordered(8);

    (entity_send, stream)
}

//
// async fn samble_clippyd(entity: &mut MainEntity) {
//     let Some(res) = sambl_get_album_for_entity(&ALISTRAL_CLIENT.symphonize, entity)
//         .await
//         .unwrap()
//     else {
//         return;
//     };

//     stream::iter(res.album_data).for_each(async |album| {});

//     let mut lints = MissingSamblReleaseLint::get_all_lints(&ALISTRAL_CLIENT.symphonize, entity)
//         .await
//         .unwrap();

//     while let Some((album, lint)) = lints.try_next().await.unwrap() {
//         if let Some(lint) = lint {
//             print_lint(&lint).await;
//         }
//     }
// }

async fn process_album(artist: Arc<Artist>, album: AlbumData, filter: &WhitelistBlacklist<String>) {
    process_lint::<MissingSamblReleaseLint>(&artist, &album, filter).await;

    let _lock = PRINT_LOCK
        .acquire()
        .await
        .expect("Print lock has been closed");

    info!(
        "[Processed - {}] [SAMBL] `{}`",
        PROCESSED_COUNT.fetch_add(1, Ordering::AcqRel),
        album.name
    );
}

// async fn process_lint<L: SamblLint>(
//     artist: &Artist,
//     album: &AlbumData,
//     filter: &WhitelistBlacklist<String>,
// ) {
//     // Check if the lint is allowed
//     if !filter.is_allowed(&L::get_name().to_string()) {
//         return;
//     }

//     let Some(_lint) = L::check_album_data(&ALISTRAL_CLIENT.symphonize, artist, album)
//         .await
//         .expect("Error while processing lint")
//     else {
//         return;
//     };

//     // There might be an issue, so grab the latest data and recheck
//     // Also prevent others from fetching data that might get stale after the user fix this lint

//     debug!(
//         "Rechecking Lint `{}` for `{}`",
//         L::get_name(),
//         album.spotify_name
//     );

//     let _lock = REFETCH_LOCK
//         .acquire()
//         .await
//         .expect("Refetch lock has been closed");

//     L::refresh_album_data(&ALISTRAL_CLIENT.symphonize, album)
//         .await
//         .expect("Couldn't refresh the entity");

//     let Some(lint) = L::check_album_data(&ALISTRAL_CLIENT.symphonize, artist, album)
//         .await
//         .expect("Error while processing lint")
//     else {
//         return;
//     };

//     print_lint(&lint).await;
// }

async fn process_lint<L: SamblLint>(
    artist: &Artist,
    album: &AlbumData,
    filter: &WhitelistBlacklist<String>,
) {
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
        return;
    }

    // Check the lint with old data

    debug!(
        "Checking Lint `{}` for `{}`",
        L::get_name(),
        album.name
    );

    if L::check_album_data(&ALISTRAL_CLIENT.symphonize, artist, album)
        .await
        .expect("Error while processing lint")
        .is_none()
    {
        return;
    }

    // There might be an issue, so grab the latest data and recheck
    // Also prevent others from fetching data that might get stale after the user fix this lint

    let _lock = REFETCH_LOCK
        .acquire()
        .await
        .expect("Refetch lock has been closed");

    let Some(lint) = recheck_lint::<L>(artist, album).await else {
        return;
    };

    print_lint(&lint).await;
}

//#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn recheck_lint<L: SamblLint>(artist: &Artist, album: &AlbumData) -> Option<L> {
    //pg_spinner!("Verifying lint...");

    debug!(
        "Rechecking Lint `{}` for `{}`",
        L::get_name(),
        album.name
    );

    L::refresh_album_data(&ALISTRAL_CLIENT.symphonize, album)
        .await
        .expect("Couldn't refresh the entity");

    L::check_album_data(&ALISTRAL_CLIENT.symphonize, artist, album)
        .await
        .expect("Error while processing lint")
}

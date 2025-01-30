use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use tracing::info;

use crate::api::clients::ALISTRAL_CLIENT;

/// Prefetch all the recordings of a list of listens
pub async fn prefetch_recordings_of_listens(
    conn: &mut sqlx::SqliteConnection,
    user_id: i64,
    listens: &[Listen],
) -> Result<(), musicbrainz_db_lite::Error> {
    let recordings = Listen::get_unfetched_recordings_ids(conn, user_id, listens).await?;

    info!("Fetching missing recording data");
    for recording in recordings {
        Recording::get_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db, &recording).await?;
    }

    Ok(())
}

/// Prefetch all the release of a list of recordings
pub async fn prefetch_releases(
    conn: &mut sqlx::SqliteConnection,
    releases: &[&Release],
) -> Result<(), musicbrainz_db_lite::Error> {
    // Eliminate all the recordings that are complete
    let uncompletes = releases
        .iter()
        .filter(|r| !r.is_fully_fetched())
        .collect_vec();

    info!("Fetching missing release data");
    for release in uncompletes {
        release
            .fetch_if_incomplete(conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await?;
    }

    Ok(())
}

pub async fn fetch_recordings_as_complete(
    conn: &mut sqlx::SqliteConnection,
    recordings: &[&Recording],
) -> Result<(), musicbrainz_db_lite::Error> {
    // Eliminate all the recordings that are complete
    let uncompletes = recordings
        .iter()
        .filter(|r| !r.is_fully_fetched())
        .collect_vec();

    info!("Fetching missing recording data");
    for recording in uncompletes {
        recording
            .fetch_if_incomplete(conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await?;
    }

    Ok(())
}

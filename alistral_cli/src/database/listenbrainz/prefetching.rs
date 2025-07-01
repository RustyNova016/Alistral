use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;

use crate::ALISTRAL_CLIENT;

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

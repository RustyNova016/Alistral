use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::cli::logger::println_cli;
use crate::cli::progress_bar::global_progress_bar::PG_FETCHING;

/// Prefetch all the recordings of a list of listens
pub async fn prefetch_recordings_of_listens(
    conn: &mut sqlx::SqliteConnection,
    client: &crate::AlistralClient,
    user_id: i64,
    listens: &[Listen],
) -> Result<(), musicbrainz_db_lite::Error> {
    let recordings = Listen::get_unfetched_recordings_ids(conn, user_id, listens).await?;
    let progress_bar = PG_FETCHING.get_submitter(recordings.len() as u64);

    println_cli("Fetching missing recording data");
    for recording in recordings {
        Recording::get_or_fetch(conn, &client.musicbrainz_db, &recording).await?;
        progress_bar.inc(1);
    }

    Ok(())
}

pub async fn fetch_recordings_as_complete(
    conn: &mut sqlx::SqliteConnection,
    client: &crate::AlistralClient,
    recordings: &[&Recording],
) -> Result<(), musicbrainz_db_lite::Error> {
    // Eliminate all the recordings that are complete
    let uncompletes = recordings
        .iter()
        .filter(|r| !r.is_fully_fetched())
        .collect_vec();

    let progress_bar = PG_FETCHING.get_submitter(uncompletes.len() as u64);

    println_cli("Fetching missing recording data");
    for recording in uncompletes {
        recording
            .fetch_if_incomplete(conn, &client.musicbrainz_db)
            .await?;
        progress_bar.inc(1);
    }

    Ok(())
}

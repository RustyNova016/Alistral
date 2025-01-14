use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::cli::logger::println_cli;
use crate::cli::progress_bar::global_progress_bar::PG_FETCHING;

pub async fn fetch_recordings_as_complete(
    conn: &mut sqlx::SqliteConnection,
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
        recording.fetch_if_incomplete(conn).await?;
        progress_bar.inc(1);
    }

    Ok(())
}

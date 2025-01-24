use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::cli::logger::println_cli;
use crate::cli::progress_bar::global_progress_bar::PG_FETCHING;

/// Prefetch all the release of a list of recordings
pub async fn prefetch_releases(
    conn: &mut sqlx::SqliteConnection,
    client: &crate::AlistralClient,
    releases: &[&Release],
) -> Result<(), musicbrainz_db_lite::Error> {
    // Eliminate all the recordings that are complete
    let uncompletes = releases
        .iter()
        .filter(|r| !r.is_fully_fetched())
        .collect_vec();

    let progress_bar = PG_FETCHING.get_submitter(uncompletes.len() as u64);

    println_cli("Fetching missing release data");
    for release in uncompletes {
        release
            .fetch_if_incomplete(conn, &client.musicbrainz_db)
            .await?;
        progress_bar.inc(1);
    }

    Ok(())
}

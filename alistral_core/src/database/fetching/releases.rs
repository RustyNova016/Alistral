use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use tracing::Span;
use tracing::info;
use tracing::instrument;
use tracing_indicatif::span_ext::IndicatifSpanExt as _;
use tuillez::pg_counted;

/// Prefetch all the release of a list of recordings
#[instrument(skip(client, conn, releases), fields(indicatif.pb_show = tracing::field::Empty))]
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

    pg_counted!(uncompletes.len(), "Fetching releases");
    info!("Fetching full release data");

    for release in uncompletes {
        release
            .fetch_if_incomplete(conn, &client.musicbrainz_db)
            .await?;
        Span::current().pb_inc(1);
    }

    Ok(())
}

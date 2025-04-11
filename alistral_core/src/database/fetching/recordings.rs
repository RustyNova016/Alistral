use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;
use tracing::instrument;
use tracing::Span;
use tracing_indicatif::span_ext::IndicatifSpanExt;
use tuillez::pg_counted;

/// Prefetch all the recordings of a list of listens
#[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn prefetch_recordings_of_listens(
    conn: &mut sqlx::SqliteConnection,
    client: &crate::AlistralClient,
    user_id: i64,
    listens: &[Listen],
) -> Result<(), musicbrainz_db_lite::Error> {
    let recordings = Listen::get_unfetched_recordings_ids(conn, user_id, listens).await?;
    pg_counted!(recordings.len(), "Fetching recordings");

    for recording in recordings {
        Recording::get_or_fetch(conn, &client.musicbrainz_db, &recording).await?;
        Span::current().pb_inc(1);
    }

    Ok(())
}

#[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn fetch_recordings_as_complete(
    client: &crate::AlistralClient,
    recordings: &[&Recording],
) -> Result<(), musicbrainz_db_lite::Error> {
    // Eliminate all the recordings that are complete
    let uncompletes = recordings
        .iter()
        .filter(|r| !r.is_fully_fetched())
        .collect_vec();

    pg_counted!(uncompletes.len(), "Fetching recordings");
    info!("Fetching full recording data");

    let conn = &mut *client.musicbrainz_db.get_raw_connection().await?;

    for recording in uncompletes {
        recording
            .fetch_if_incomplete(conn, &client.musicbrainz_db)
            .await?;
        Span::current().pb_inc(1);
    }

    Ok(())
}

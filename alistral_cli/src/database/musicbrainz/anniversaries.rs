use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

pub async fn get_recordings_aniversaries(
    conn: &mut sqlx::SqliteConnection,
    date: &DateTime<Utc>,
) -> Result<Vec<Recording>, crate::Error> {
    let month = date.month();
    let day = date.day();

    Ok(
        sqlx::query_as!(
            Recording,
            "SELECT
                * 
            FROM
                recordings
            WHERE
                CAST(strftime('%m', DATE(recordings.first_release_date, 'unixepoch')) AS INTEGER) = ?
                AND
                CAST(strftime('%d', DATE(recordings.first_release_date, 'unixepoch')) AS INTEGER) = ?",
                month,
                day
            ).fetch_all(conn).await?)
}

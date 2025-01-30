use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::utils::cli::display::RecordingExt;
use crate::utils::println_cli;

pub async fn refresh_data(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    limit: i64,
    max_ts: i64,
) {
    let _ = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(&mut *conn)
        .await
        .expect("Couldn't fetch the new listens");

    let mbids = sqlx::query_scalar!("
            SELECT DISTINCT
                recordings.mbid
            FROM
                listens
                INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
                INNER JOIN recordings ON recordings_gid_redirect.new_id = recordings.id
                INNER JOIN users ON listens.user = users.name
            WHERE
                msid_mapping.user = users.id
                AND LOWER(users.name) = LOWER(?)
                AND `full_update_date` < ?
            ORDER BY `full_update_date` DESC
            LIMIT ?", username,max_ts,  limit
        ).fetch_all(&mut *conn).await.expect("Couldn't retrieve the listened mbids");

    pg_counted!(mbids.len(), "Refetching data");
    for mbid in mbids {
        let recording = Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, &mbid)
            .await
            .expect("Couldn't refresh mbid");

        pg_inc!();

        if let Some(recording) = recording {
            // It's ok to silently discard the error here. It's just some fancy display
            if let Ok(recording) = recording.pretty_format_with_credits(conn, true).await {
                println_cli(format!("Refreshed: {recording}"));
            }
        }
    }
}

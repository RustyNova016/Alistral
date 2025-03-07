use chrono::{DateTime, Utc};
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use tracing::info;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::ALISTRAL_CLIENT;

/// Fetch the latest listens for the provided user. If the user has no listens, it will do a full listen fetch.
#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn fetch_latest_listens_of_user(
    conn: &mut sqlx::SqliteConnection,
    user: &str,
) -> Result<(), musicbrainz_db_lite::Error> {
    pg_spinner!("Loading listens of {}", user);

    info!("Fetching recordings from listens");

    let latest_listen_ts = Listen::get_latest_listen_of_user(&mut *conn, user)
        .await?
        .map(|v| v.listened_at);
    let mut pull_ts = Some(Utc::now().timestamp());

    // This loop has two possible states.
    // - Fresh dump:
    //     `latest_listen_ts` is none. We loop until `save_listen_payload_in_transaction` tell us it's over
    //
    // - Incremental dump:
    //     `latest_listen_ts` is set. We loop until pull_ts is before the latest listen
    while (latest_listen_ts.is_none() && pull_ts.is_some())
        || (latest_listen_ts.is_some_and(|a| pull_ts.is_some_and(|b| a <= b)))
    {
        info!(
            "Getting listens from before: {} ({})",
            DateTime::from_timestamp(pull_ts.unwrap(), 0).unwrap(),
            pull_ts.unwrap()
        );
        pull_ts = Listen::execute_listen_fetch(
            conn,
            &ALISTRAL_CLIENT.listenbrainz,
            user,
            pull_ts.unwrap(),
        )
        .await?;
    }

    Ok(())
}

use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;
use tuillez::fatal_error::FatalError;
use tuillez::fatal_error::IntoFatal;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;

#[cfg(not(test))]
use crate::utils::cli::await_next;

pub async fn lookup_recording(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    id: &str,
) -> Result<(), FatalError> {
    // Fetch the listens.
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn, &ALISTRAL_CLIENT.core)
        .await
        .expect_fatal("Couldn't fetch listens")?;

    // Refetch the recording to make sure it's up to date
    let Some(recording) = Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, id)
        .await
        .map_err(crate::Error::from)?
    else {
        info!("Couldn't find the recording with id: {id}");
        return Ok(());
    };

    let mut all_listens = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Couldn't load recordings");

    let target_recording = all_listens.get_or_new(recording).clone();

    println!(
        "{}",
        target_recording
            .get_lookup_report(conn, &all_listens)
            .await
            .expect("Couldn't generate lookup report")
    );

    #[cfg(not(test))]
    await_next();

    Ok(())
}

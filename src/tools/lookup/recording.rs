use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::database::get_db_client;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListensOld;
use crate::utils::println_cli;

#[cfg(not(test))]
use crate::utils::cli::await_next;

pub async fn lookup_recording(username: &str, id: &str) -> color_eyre::Result<()> {
    let db = get_db_client().await;
    let conn = &mut *db.connection.acquire().await?;

    // Fetch the listens.
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await?;

    // Refetch the recording to make sure it's up to date
    let Some(recording) = Recording::fetch_and_save(conn, id).await? else {
        println_cli(format!("Couldn't find the recording with id: {id}"));
        return Ok(());
    };

    let mut all_listens = RecordingWithListensOld::from_listencollection(conn, listens)
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

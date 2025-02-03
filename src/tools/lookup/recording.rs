use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_timeframe::time_windows::TimeWindow;
use alistral_core::datastructures::listen_timeframe::timeframe_settings::TimeframeSettings;
use chrono::DateTime;
use chrono::Duration;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::models::lookup_reports::recordings::RecordingLookupReport;

#[cfg(not(test))]
use crate::utils::cli::await_next;

pub async fn lookup_recording(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    id: &str,
) -> color_eyre::Result<()> {
    // Fetch the listens.
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await?;

    // Refetch the recording to make sure it's up to date
    let Some(recording) =
        Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, id).await?
    else {
        info!("Couldn't find the recording with id: {id}");
        return Ok(());
    };

    let all_listens =
        RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT, listens)
            .await
            .expect("Couldn't load recordings");

    let timeframe = TimeframeSettings::new(
        TimeWindow::new(
            DateTime::from_timestamp(1735689601, 0).unwrap(),
            DateTime::from_timestamp(1738368001, 0).unwrap(),
        ),
        true,
        true,
    );
    let report = RecordingLookupReport::new(all_listens, recording, timeframe, false);

    println!(
        "{}",
        report
            .generate_full_report(conn)
            .await
            .expect("Couldn't generate lookup report")
    );

    // let mut all_listens = RecordingWithListens::from_listencollection(conn, listens)
    //     .await
    //     .expect("Couldn't load recordings");

    // let target_recording = all_listens.get_or_new(recording).clone();

    // println!(
    //     "{}",
    //     target_recording
    //         .get_lookup_report(conn, &all_listens)
    //         .await
    //         .expect("Couldn't generate lookup report")
    // );

    #[cfg(not(test))]
    await_next();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::clients::ALISTRAL_CLIENT;
    use crate::tools::lookup::recording::lookup_recording;
    #[sqlx::test]
    async fn report_test() {
        let conn = &mut *ALISTRAL_CLIENT
            .musicbrainz_db
            .connection
            .acquire_guarded()
            .await;

        lookup_recording(conn, "RustyNova", "ff0c4bd4-de61-4512-b5c0-be4edb81d2fc")
            .await
            .unwrap()
    }
}

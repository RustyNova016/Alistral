use sequelles::SelectUnique;
use sequelles::Upsert;
use sqlx::SqlitePool;

use crate::Recording;
use crate::models::musicbrainz::recording::RecordingMbid;

#[sqlx::test(migrator = "crate::MIGRATOR")]
pub fn upsert_recording(pool: SqlitePool) {
    let conn = &mut *pool.acquire().await.unwrap();

    let recording = Recording {
        id: 0,
        annotation: None,
        artist_credit: None,
        disambiguation: None,
        first_release_date: None,
        full_update_date: None,
        length: None,
        video: None,
        mbid: "test".to_string(),
        title: "test_upsert_recording".to_string(),
    };

    recording.upsert(conn).await.unwrap();

    Recording::select_unique(
        conn,
        RecordingMbid {
            mbid: "test".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

       let recording = Recording {
        id: 0,
        annotation: None,
        artist_credit: None,
        disambiguation: None,
        first_release_date: None,
        full_update_date: None,
        length: None,
        video: None,
        mbid: "test".to_string(),
        title: "test_upsert_recording_ren".to_string(),
    };

    // recording.upsert(conn).await.unwrap();

    // let rec =     Recording::select_unique(
    //     conn,
    //     RecordingMbid {
    //         mbid: "test".to_string(),
    //     },
    // )
    // .await
    // .unwrap()
    // .unwrap();

    
}

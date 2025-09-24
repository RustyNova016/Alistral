pub mod database;
use core::fmt::Display;

use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::HasArtistCredits as _;
use sqlx::prelude::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct MessyRecording {
    pub id: i64,
    pub title: String,
    pub artist_credits: String,
    pub release: Option<String>,
    pub mbid: Option<String>,
}

impl MessyRecording {
    pub async fn from_mbid_with_db(
        db_lite_conn: &mut sqlx::SqliteConnection,
        client: &crate::InterzicClient,
        mbid: &str,
    ) -> Result<Self, crate::Error> {
        let db_lite = client.musicbrainz_db_lite_client()?;
        let recording = Recording::get_or_fetch(db_lite_conn, db_lite, mbid)
            .await?
            .unwrap(); //TODO: Replace when https://github.com/RustyNova016/musicbrainz_db_lite/issues/63 is done
        Self::from_db_recording(db_lite_conn, client, recording).await
    }

    pub async fn from_db_recording(
        db_lite_conn: &mut sqlx::SqliteConnection,
        client: &crate::InterzicClient,
        recording: Recording,
    ) -> Result<Self, crate::Error> {
        let db_lite = client.musicbrainz_db_lite_client()?;

        let credits = recording
            .get_artist_credits_or_fetch(db_lite_conn, db_lite)
            .await?;
        let release = recording
            .get_releases_or_fetch(db_lite_conn, db_lite)
            .await?;

        Ok(MessyRecording {
            id: 0,
            mbid: Some(recording.mbid),
            title: recording.title,
            artist_credits: credits.to_string(),
            release: release.into_iter().next().map(|r| r.title),
        })
    }
}

impl Display for MessyRecording {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.artist_credits, self.title)?;

        if !self.release.as_ref().is_none_or(|r| r.is_empty()) {
            write!(f, " ({})", self.release.clone().unwrap())?;
        }
        Ok(())
    }
}

use core::fmt::Display;

use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
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
    pub async fn from_db_recording(
        conn: &mut sqlx::SqliteConnection,
        recording: Recording,
    ) -> Result<Self, crate::Error> {
        let credits = recording.get_artist_credits_or_fetch(conn).await?;
        let release = recording.get_releases_or_fetch(conn).await?;

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

use core::fmt::Display;

use sqlx::prelude::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct MessyRecording {
    pub id: i64,
    pub title: String,
    pub artist_credits: String,
    pub release: Option<String>,
    pub mbid: Option<String>,
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
use sequelles::has_rowid::HasRowID;
use sqlx::prelude::FromRow;
use sqlx::{Executor, Sqlite};

pub mod finds;
pub mod relations;
pub mod selects;

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, FromRow, Clone, bon::Builder)]
pub struct MessybrainzSubmission {
    pub id: i64,
    pub msid: String,
    pub recording: String,
    pub artist_credit: String,
    pub release: Option<String>,
    pub track_number: Option<String>,
    pub duration: Option<i64>,
}

impl MessybrainzSubmission {
    pub async fn insert_or_ignore(
        &self,
        client: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO `messybrainz_submission` VALUES (NULL, ?, ?, ?, ?, ?, ?)",
            self.msid,
            self.recording,
            self.artist_credit,
            self.release,
            self.track_number,
            self.duration
        )
        .execute(client)
        .await?;
        Ok(())
    }
}

impl HasRowID for MessybrainzSubmission {
    fn rowid(&self) -> i64 {
        self.id
    }
}

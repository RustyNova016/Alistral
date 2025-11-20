use sequelles::has_rowid::HasRowID;
use sqlx::prelude::FromRow;

pub mod finds;
pub mod relations;
pub mod selects;

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, FromRow, Clone, bon::Builder)]
pub struct MessybrainzSubmission {
    pub id: i64,
    #[builder(into)]
    pub msid: String,
    #[builder(into)]
    pub recording: String,
    #[builder(into)]
    pub artist_credit: String,
    #[builder(into)]
    pub release: Option<String>,
    #[builder(into)]
    pub track_number: Option<String>,
    pub duration: Option<i64>,
}

impl MessybrainzSubmission {
    pub async fn insert_or_ignore(
        &self,
        conn: &mut sqlx::SqliteConnection,
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
        .execute(conn)
        .await?;
        Ok(())
    }
}

impl HasRowID for MessybrainzSubmission {
    fn rowid(&self) -> i64 {
        self.id
    }
}

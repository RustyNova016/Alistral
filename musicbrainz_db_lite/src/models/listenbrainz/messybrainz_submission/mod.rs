use macon::Builder;
use sqlx::prelude::FromRow;
use sqlx::{Executor, Sqlite};

use crate::RowId;

pub mod finds;
pub mod relations;
pub mod selects;

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, Builder, FromRow, Clone)]
#[builder(Default=!)]
pub struct MessybrainzSubmission {
    pub id: i64,

    #[builder(Default=!)]
    pub msid: String,

    #[builder(Default=!)]
    pub recording: String,

    #[builder(Default=!)]
    pub artist_credit: String,

    #[builder(Default=!)]
    pub release: Option<String>,

    #[builder(Default=!)]
    pub track_number: Option<String>,

    #[builder(Default=!)]
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

impl RowId for MessybrainzSubmission {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

use crate::models::shared_traits::find_by::FindBy;
use crate::models::shared_traits::find_by_rowid::FindByRowID;

use super::MessybrainzSubmission;

impl MessybrainzSubmission {
    pub async fn find_by_msid(
        conn: &mut sqlx::SqliteConnection,
        id: String,
    ) -> Result<Option<Self>, crate::Error> {
        Self::find_by(conn, id).await
    }
}

impl FindByRowID for MessybrainzSubmission {
    async fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(sqlx::query_as!(
            MessybrainzSubmission,
            "SELECT * FROM `messybrainz_submission` WHERE id = ?",
            id
        )
        .fetch_optional(conn)
        .await?)
    }
}

impl FindBy<String> for MessybrainzSubmission {
    async fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: String,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(sqlx::query_as!(
            MessybrainzSubmission,
            "SELECT * FROM `messybrainz_submission` WHERE msid = ?",
            id
        )
        .fetch_optional(conn)
        .await?)
    }
}

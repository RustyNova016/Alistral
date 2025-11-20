use snafu::ResultExt as _;

use crate::MBIDRedirection;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::shared_traits::find_by_mbid::FindByMBID;
use crate::models::shared_traits::find_by_rowid::FindByRowID;

use super::Label;

impl FindByRowID for Label {
    async fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM `labels` WHERE `id` = $1", id)
                .fetch_optional(conn)
                .await
                .context(SqlxSnafu)?,
        )
    }
}

impl FindByMBID for Label {
    async fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        id: &str,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(<Self as MBIDRedirection>::find_by_mbid(conn, id).await?)
    }
}

use crate::models::shared_traits::find_by_mbid::FindByMBID;
use crate::models::shared_traits::find_by_rowid::FindByRowID;

use super::Label;

impl FindByRowID for Label {
    async fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(Self::find_by_id_column(conn, id).await?)
    }
}

impl FindByMBID for Label {
    async fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        id: &str,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(Self::find_by_mbid(conn, id).await?)
    }
}

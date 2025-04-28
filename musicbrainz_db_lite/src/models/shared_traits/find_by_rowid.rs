use crate::models::RowID;

use super::RowId;
use super::find_by::FindBy;

pub trait FindByRowID: RowId + Sized {
    fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;
}

impl<T: FindByRowID> FindBy<RowID> for T {
    async fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: RowID,
    ) -> Result<Option<Self>, crate::Error> {
        Self::find_by_rowid(conn, *id).await
    }
}

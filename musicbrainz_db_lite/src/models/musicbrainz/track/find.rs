use crate::FindBy;
use crate::Track;
use crate::models::RowID;
use crate::models::shared_traits::find_by_mbid::MBID;

impl FindBy<RowID> for Track {
    async fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: RowID,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(sqlx::query_as("SELECT * FROM track WHERE id = ?")
            .bind(*id)
            .fetch_optional(conn)
            .await?)
    }
}

impl FindBy<MBID> for Track {
    async fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: MBID,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(sqlx::query_as("SELECT * FROM track WHERE gid = ?")
            .bind(id.to_string())
            .fetch_optional(conn)
            .await?)
    }
}

impl Track {
    pub async fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: String,
    ) -> Result<Option<Self>, crate::Error> {
        <Self as FindBy<MBID>>::find_by(conn, MBID(mbid)).await
    }
}

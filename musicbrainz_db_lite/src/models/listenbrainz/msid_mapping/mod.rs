use sqlx::{Executor, Sqlite, SqliteConnection};

#[derive(Debug, Clone)]
pub struct MsidMapping {
    pub id: i64,

    pub recording_mbid: String,

    pub recording_msid: String,

    pub release_mbid: Option<String>,

    pub user: i64,
}

impl MsidMapping {
    pub async fn find_by_user_msid(
        conn: &mut SqliteConnection,
        user_id: i64,
        msid: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM msid_mapping WHERE recording_msid = ? AND user = ?;",
            msid,
            user_id
        )
        .fetch_optional(conn)
        .await
    }

    /// Set the MBID mapping for an msid for user
    pub async fn set_user_mapping(
        client: impl Executor<'_, Database = Sqlite>,
        user_id: i64,
        msid: String,
        mbid: String,
    ) -> Result<(), sqlx::Error> {
        //println!("mapping {} to {}", msid, mbid);

        sqlx::query!("INSERT INTO `msid_mapping` VALUES (NULL, ?, ?, ?, NULL) ON CONFLICT DO UPDATE SET `recording_mbid` = ?", msid, mbid, user_id, mbid).execute(client).await?;
        Ok(())
    }
}

use sqlx::FromRow;
use sqlx::sqlite::SqliteRow;

use crate::RowId;
use crate::models::shared_traits::has_table::HasTable;

pub trait MBIDRedirection
where
    Self: HasTable + RowId,
{
    /// Return the name of the mbid redirect table for the current entity
    fn get_redirect_table_name() -> String {
        format!("{}_gid_redirect", Self::TABLE_NAME)
    }

    /// Insert a mbid in the mbid redirection table. It doesn't link to any actual entity
    fn add_redirect_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
            sqlx::query(&format!(
                "INSERT OR IGNORE INTO `{}_gid_redirect` VALUES (?, NULL, 0)",
                Self::TABLE_NAME
            ))
            .bind(mbid)
            .execute(conn)
            .await?;
            Ok(())
        }
    }

    /// Link an mbid to the actual entity
    fn link_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
            sqlx::query(&format!(
                "INSERT OR IGNORE INTO `{}_gid_redirect` VALUES (?, ?, 0) ON CONFLICT DO UPDATE SET `new_id` = ?",
                Self::TABLE_NAME
            ))
        .bind(mbid)
        .bind(id)
        .bind(id)
        .execute(conn)
        .await?;
            Ok(())
        }
    }

    fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Self: Sized + for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
    {
        async move {
            sqlx::query_as(&format!(
                r#"
                    SELECT
                        {table}.*
                    FROM
                        {table}
                        INNER JOIN {table}_gid_redirect ON {table}.id = {table}_gid_redirect.new_id
                    WHERE
                        {table}_gid_redirect.gid = ?
                        AND deleted = 0
                    LIMIT
                        1
                "#,
                table = Self::TABLE_NAME
            ))
            .bind(mbid)
            .fetch_optional(conn)
            .await
        }
    }

    /// Return all the mbid aliases from an entity's id
    fn get_mbid_aliases_of_id(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> impl std::future::Future<Output = Result<Vec<String>, sqlx::Error>> + Send {
        async move {
            sqlx::query_scalar(&format!(
                "SELECT gid FROM `{}_gid_redirect` WHERE new_id = ?",
                Self::TABLE_NAME
            ))
            .bind(id)
            .fetch_all(conn)
            .await
        }
    }

    /// Return all the mbid aliases of an mbid
    fn get_mbid_aliases_of_mbid(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Vec<String>, sqlx::Error>> + Send {
        async move {
            sqlx::query_scalar(&format!(
                    "SELECT gid FROM `{entity}_gid_redirect` WHERE new_id = (SELECT new_id FROM `{entity}_gid_redirect` WHERE gid = ?)",
                    entity = Self::TABLE_NAME
                ))
                .bind(mbid)
                .fetch_all(conn)
                .await
        }
    }

    // === Self methods ===

    /// Return all the mbid aliases of the current entity
    fn get_mbids_aliases(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Vec<String>, sqlx::Error>> + Send {
        let id = self.get_row_id();
        async move { Self::get_mbid_aliases_of_id(conn, id).await }
    }
}

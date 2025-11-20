use snafu::ResultExt;
use sqlx::SqliteConnection;

use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;

use super::Listen;

//TODO: Unify everything into an **exclusive** function
//TODO: Remove macro

impl Listen {
    /// Delete a range of listens. The start and end timestamps are **exclusive**. Listens at start_ts **won't** be deleted.
    ///
    /// Start timestamp is the lower date, end timestamp is the higher one
    pub async fn delete_listen_range(
        conn: &mut SqliteConnection,
        start_ts: i64,
        end_ts: i64,
        username: &str,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, SqlxError> {
        sqlx::query!(
            "DELETE FROM `listens` WHERE ? < listened_at AND listened_at < ? AND `user` = ?",
            start_ts,
            end_ts,
            username
        )
        .execute(conn)
        .await
        .context(SqlxSnafu)
    }

    /// Delete a periode of listens. The start and end timestamps are **inclusive**.
    ///
    /// Start timestamp is the lower date, end timestamp is the higher one
    pub async fn delete_listen_period(
        conn: &mut SqliteConnection,
        start_ts: i64,
        end_ts: i64,
        username: &str,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        sqlx::query!(
            "DELETE FROM `listens` WHERE listened_at >= ? AND listened_at <= ? AND `user` = ?",
            start_ts,
            end_ts,
            username
        )
        .execute(conn)
        .await
    }
}

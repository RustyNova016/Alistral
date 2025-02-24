use sqlx::prelude::FromRow;

use crate::models::shared_traits::has_tags::HasTags;
use crate::RowId;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub count: Option<i64>,
    pub score: Option<i64>,
}

impl Tag {
    pub async fn upsert<T: HasTags>(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        foreign_key: i64,
    ) -> Result<(), crate::Error> {
        let returned: Tag = sqlx::query_as(&format!(
            "
        INSERT INTO
            `{}_tag` (
                `name`,
                `count`,
                `score`,
                `{}`
            )
        VALUES
            (?, ?, ?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `count` = excluded.`count`,
            `score` = excluded.`score`
        RETURNING *;",
            T::TABLE_NAME,
            T::FOREIGN_FIELD_NAME
        ))
        .bind(&self.name)
        .bind(self.count)
        .bind(self.score)
        .bind(foreign_key)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }
}

impl RowId for Tag {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

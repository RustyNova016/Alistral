use sqlx::FromRow;

use crate::models::shared_traits::has_genre::HasGenres;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct GenreTag {
    pub id: i64,
    pub count: Option<i64>,

    pub genre: i64,
}

impl GenreTag {
    pub async fn upsert<T: HasGenres>(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        foreign_key: i64,
    ) -> Result<(), crate::Error> {
        let returned = sqlx::query_as(&format!(
            "
        INSERT INTO
            `{}_genre` (
                `count`,
                `{}`,
                `genre`
            )
        VALUES
            (?, ?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `count` = excluded.`count`
        RETURNING *;",
            T::TABLE_NAME,
            T::FOREIGN_FIELD_NAME
        ))
        .bind(self.count)
        .bind(foreign_key)
        .bind(self.genre)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }
}

pub mod finds;
pub mod genre_tag;
use sqlx::prelude::FromRow;

use crate::models::shared_traits::has_table::HasTable;
use crate::RowId;

use super::relations::impl_relations::impl_relations;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct Genre {
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub disambiguation: Option<String>,
}

impl Genre {
    pub async fn upsert(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let returned: Genre = sqlx::query_as(
            "
        INSERT INTO
            `genres` (
                `mbid`,
                `name`,
                `disambiguation`
            )
        VALUES
            (?, ?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `disambiguation` = excluded.`disambiguation`
        RETURNING *;",
        )
        .bind(&self.mbid)
        .bind(&self.name)
        .bind(&self.disambiguation)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }
}

impl_relations!(Genre);

impl RowId for Genre {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for Genre {
    const TABLE_NAME: &str = "genres";

    const FOREIGN_FIELD_NAME: &str = "genre";
}

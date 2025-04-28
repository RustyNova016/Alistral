use sqlx::FromRow;
use sqlx::sqlite::SqliteRow;

use crate::models::musicbrainz::relations::Relation;
use crate::models::musicbrainz::relations::traits::HasRelation;
use crate::models::shared_traits::has_table::HasTable;

impl<T, U> Relation<T, U>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin + HasRelation<U>,
    U: for<'a> FromRow<'a, SqliteRow> + HasTable + Send + Unpin + Clone,
{
    async fn get_entity_inner<V>(
        &self,
        conn: &mut sqlx::SqliteConnection,
        entity_num: &str,
    ) -> Result<V, crate::Error>
    where
        V: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
    {
        Ok(sqlx::query_as(&format!(
            r#"SELECT
                        right.*
                    FROM
                        {right_table} as right
                        INNER JOIN {left_table} as left ON right.id = left.entity{entity_num}
                    WHERE
                        left.id = ?"#,
            left_table = T::RELATION_TABLE,
            right_table = U::TABLE_NAME
        ))
        .bind(self.id)
        .fetch_one(conn)
        .await?)
    }

    /// Get the entity0 of the relation, infering it as a T type
    pub async fn get_entity_0_as_left(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<T, crate::Error> {
        self.get_entity_inner(conn, "0").await
    }

    /// Get the entity0 of the relation, infering it as a U type
    pub async fn get_entity_0_as_right(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<U, crate::Error> {
        self.get_entity_inner(conn, "0").await
    }

    /// Get the entity1 of the relation, infering it as a T type
    pub async fn get_entity_1_as_left(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<T, crate::Error> {
        self.get_entity_inner(conn, "1").await
    }

    /// Get the entity1 of the relation, infering it as a U type
    pub async fn get_entity_1_as_right(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<U, crate::Error> {
        self.get_entity_inner(conn, "1").await
    }
}

impl<T> Relation<T, T>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin + HasRelation<T>,
{
    /// The other entity of the relationship
    pub async fn get_other_entity(
        &self,
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<T, crate::Error> {
        if self.entity0 == id {
            self.get_entity_1_as_left(conn).await
        } else {
            self.get_entity_0_as_left(conn).await
        }
    }
}

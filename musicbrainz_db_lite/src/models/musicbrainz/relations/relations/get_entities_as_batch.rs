use std::collections::HashMap;

use itertools::Itertools as _;
use sqlx::FromRow;
use sqlx::sqlite::SqliteRow;

use crate::models::musicbrainz::relations::Relation;
use crate::models::musicbrainz::relations::traits::HasRelation;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

impl<T, U> Relation<T, U>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin + HasRelation<U> + Clone,
    U: for<'a> FromRow<'a, SqliteRow> + HasTable + Send + Unpin + Clone,
{
    async fn get_entity_as_batch_inner<'r, V>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
        entity_num: &str,
    ) -> Result<HashMap<i64, (&'r &'r Self, Vec<V>)>, crate::Error>
    where
        V: for<'a> FromRow<'a, SqliteRow> + Send + Unpin + Clone,
    {
        let ids = left_entities.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, V>> = sqlx::query_as(&format!(
            "
                SELECT
                    left.id as original_id,
                    right.*
                FROM
                    {right_table} as right
                    INNER JOIN {left_table} as left ON right.id = left.entity{entity_num}
                WHERE
                    left.id IN (
                        SELECT
                            value
                        FROM
                            JSON_EACH(?)
                    )
            ",
            left_table = T::RELATION_TABLE,
            right_table = U::TABLE_NAME,
        ))
        .bind(id_string)
        .fetch_all(conn)
        .await?;

        Ok(JoinCollection::from(joins).into_hashmap(left_entities, |id, value| &value.id == id))
    }

    pub async fn get_entity_0_as_left_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
    ) -> Result<HashMap<i64, (&'r &'r Self, Vec<T>)>, crate::Error> {
        Self::get_entity_as_batch_inner(conn, left_entities, "0").await
    }

    pub async fn get_entity_0_as_right_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
    ) -> Result<HashMap<i64, (&'r &'r Self, Vec<U>)>, crate::Error> {
        Self::get_entity_as_batch_inner(conn, left_entities, "0").await
    }

    pub async fn get_entity_1_as_left_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
    ) -> Result<HashMap<i64, (&'r &'r Self, Vec<T>)>, crate::Error> {
        Self::get_entity_as_batch_inner(conn, left_entities, "1").await
    }

    pub async fn get_entity_1_as_right_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
    ) -> Result<HashMap<i64, (&'r &'r Self, Vec<U>)>, crate::Error> {
        Self::get_entity_as_batch_inner(conn, left_entities, "1").await
    }
}

use itertools::Itertools as _;

use crate::RowId;
use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

impl Tag {
    pub async fn query_batch<T>(
        conn: &mut sqlx::SqliteConnection,
        entities: Vec<&T>,
    ) -> Result<Vec<JoinRelation<i64, Tag>>, crate::Error>
    where
        T: RowId + HasTags,
    {
        let ids = entities.iter().map(|r| r.get_row_id()).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        Ok(sqlx::query_as(&format!(
            "
            SELECT
                {id_field} as original_id,
                {table}_tag.*
            FROM
                {table}_tag
            WHERE
                {id_field} IN (
                    SELECT
                        value
                    FROM
                        JSON_EACH(?)
                )
        ",
            id_field = T::FOREIGN_FIELD_NAME,
            table = T::TABLE_NAME
        ))
        .bind(id_string)
        .fetch_all(conn)
        .await?)
    }
}

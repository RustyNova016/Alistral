use itertools::Itertools as _;
use sqlx::sqlite::SqliteRow;

use crate::RowId;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::sqlx_utils::joins::JoinCollection;
use crate::utils::sqlx_utils::joins::JoinRelation;

pub mod complete_fetch;

/// Trait for all the entity having a database relation with another
pub trait DBRelation<Relation>
where
    Self: HasTable + RowId,
{
    type ReturnedType: HasTable + for<'a> sqlx::FromRow<'a, SqliteRow> + Send + Unpin;

    /// The sql join statement.
    fn get_join_statement() -> &'static str;

    /// Get the related entities
    fn get_related_entity(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ReturnedType>, crate::Error>> + Send
    {
        let id = self.get_row_id();
        async move {
            let query = format!(
                "SELECT
                {other}.*
                FROM
                    {this}
                    {join}
                WHERE
                    {this}.id = ?",
                this = Self::TABLE_NAME,
                other = Self::ReturnedType::TABLE_NAME,
                join = Self::get_join_statement()
            );

            Ok(sqlx::query_as(&query).bind(id).fetch_all(conn).await?)
        }
    }

    fn get_related_entity_bulk(
        conn: &mut sqlx::SqliteConnection,
        entities: &[Self],
    ) -> impl std::future::Future<Output = Result<JoinCollection<Self::ReturnedType>, crate::Error>> + Send
    where
        Self: Sized + Sync,
    {
        async {
            let ids = entities.iter().map(|r| r.get_row_id()).collect_vec();
            let id_string = serde_json::to_string(&ids)?;

            let joins: Vec<JoinRelation<Self::ReturnedType>> = sqlx::query_as(&format!(
                "
            SELECT
                {this}.id as original_id,
                {other}.*
            FROM
                {this}
                {join}
            WHERE
                {this}.id IN (
                    SELECT
                        value
                    FROM
                        JSON_EACH(?)
                )
        ",
                this = Self::TABLE_NAME,
                other = Self::ReturnedType::TABLE_NAME,
                join = Self::get_join_statement()
            ))
            .bind(id_string)
            .fetch_all(conn)
            .await?;

            Ok(joins.into())
        }
    }
}

/// Entity (0:M) -> [`crate::ArtistCredit`] (M:1) -> [`crate::Artist`]
pub struct ArtistFromCreditsRelation;

/// Recording (1:M) -> Work
pub struct RecordingWorkDBRel;

/// Entity (0:M) -> [`crate::ArtistCredit`]
pub struct ArtistCreditDBRel;

/// Entity (0:M) -> [`crate::Url`]
pub struct EntityURLDBRel;

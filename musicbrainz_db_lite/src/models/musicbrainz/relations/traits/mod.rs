pub mod self_relation;
pub trait HasRelation<U>
where
    Self: RowId + HasTable + Sized + Send + Unpin + Clone + Sync,
    U: HasTable + Send + Unpin + Clone,
{
    /// The name of the table where the relation is stored
    const RELATION_TABLE: &str;

    /// Get the row_id of the entity in the first position
    ///
    /// Ex: In a Artist -> Recording relationship, Artist is the entity0
    fn get_entity0_id(&self, other: &U) -> i64;

    /// Get the row_id of the entity in the second position
    ///
    /// Ex: In a Artist -> Recording relationship, Recording is the entity1
    fn get_entity1_id(&self, other: &U) -> i64;

    fn get_entity_relations_inner(
        &self,
        conn: &mut sqlx::SqliteConnection,
        join0: bool,
        join1: bool,
    ) -> impl std::future::Future<Output = Result<Vec<Relation<Self, U>>, crate::Error>> + Send
    {
        async move {
            let mut join = String::new();
            if join0 && join1 {
                join += "left.id = right.entity0 OR left.id = right.entity1"
            } else if join0 {
                join += "left.id = right.entity0"
            } else if join1 {
                join += "left.id = right.entity1"
            } else {
                panic!("No join has been specified!")
            }

            Ok(sqlx::query_as(&format!(
                r#"     SELECT
                        right.*
                    FROM
                        {left_table} as left
                        INNER JOIN {right_table} as right ON {join}
                    WHERE
                        left.id = ?"#,
                left_table = Self::TABLE_NAME,
                right_table = Self::RELATION_TABLE,
            ))
            .bind(self.get_row_id())
            .fetch_all(conn)
            .await?)
        }
    }

    fn get_entity_relations(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Vec<Relation<Self, U>>, crate::Error>> + Send;

    #[expect(
        clippy::type_complexity,
        reason = "Can't refactor the type as it uses Self"
    )]
    fn get_entity_relations_as_batch_inner<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
        join0: bool,
        join1: bool,
    ) -> impl std::future::Future<
        Output = Result<HashMap<i64, (&'r &'r Self, Vec<Relation<Self, U>>)>, crate::Error>,
    > + Send {
        async move {
            let ids = left_entities.iter().map(|r| r.get_row_id()).collect_vec();
            let id_string = serde_json::to_string(&ids)?;

            let mut join = String::new();
            if join0 && join1 {
                join += "left.id = right.entity0 OR left.id = right.entity1"
            } else if join0 {
                join += "left.id = right.entity0"
            } else if join1 {
                join += "left.id = right.entity1"
            } else {
                panic!("No join has been specified!")
            }

            let joins: Vec<JoinRelation<i64, Relation<Self, U>>> = sqlx::query_as(&format!(
                "
                SELECT
                    left.id as original_id,
                    right.*
                FROM
                    {left_table} as left
                    INNER JOIN {right_table} as right ON {join}
                WHERE
                    left.id IN (
                        SELECT
                            value
                        FROM
                            JSON_EACH(?)
                    )
            ",
                left_table = Self::TABLE_NAME,
                right_table = Self::RELATION_TABLE,
            ))
            .bind(id_string)
            .fetch_all(conn)
            .await?;

            Ok(JoinCollection::from(joins)
                .into_hashmap(left_entities, |id, value| &value.get_row_id() == id))
        }
    }

    #[expect(
        clippy::type_complexity,
        reason = "Can't refactor the type as it uses Self"
    )]
    fn get_entity_relations_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        left_entities: &'r [&'r Self],
    ) -> impl std::future::Future<
        Output = Result<HashMap<i64, (&'r &'r Self, Vec<Relation<Self, U>>)>, crate::Error>,
    > + Send;

    fn delete_relations_inner(
        &self,
        conn: &mut sqlx::SqliteConnection,
        entity0: bool,
        entity1: bool,
    ) -> impl std::future::Future<Output = Result<(), crate::Error>> + Send {
        async move {
            let where_clause = if entity0 && entity1 {
                "t.entity0 = $1 OR t.entity1 = $1"
            } else if entity0 {
                "t.entity0 = $1"
            } else if entity1 {
                "t.entity1 = $1"
            } else {
                panic!("No join has been specified!")
            };

            sqlx::query(&format!(
                "DELETE FROM `{}` as t WHERE {where_clause}",
                Self::RELATION_TABLE
            ))
            .bind(self.get_row_id())
            .execute(conn)
            .await?;
            Ok(())
        }
    }

    fn delete_relations(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<(), crate::Error>> + Send;
}

macro_rules! impl_has_relation {
    ($left_entity: ty, $right_entity: ty) => {
        impl crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>
            for $left_entity
        {
            const RELATION_TABLE: &str = const_format::formatcp!(
                "l_{}_{}",
                <$left_entity>::TABLE_NAME,
                <$right_entity>::TABLE_NAME
            );

            fn get_entity0_id(&self, _other: &$right_entity) -> i64 {
                self.get_row_id()
            }

            fn get_entity1_id(&self, other: &$right_entity) -> i64 {
                other.get_row_id()
            }

            async fn get_entity_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<Vec<Relation<Self, $right_entity>>, crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<
                    $right_entity,
                >>::get_entity_relations_inner(self, conn, true, false)
                .await
            }

            async fn get_entity_relations_as_batch<'r>(
                conn: &mut sqlx::SqliteConnection,
                left_entities: &'r [&'r Self],
            ) -> Result<
                HashMap<i64, (&'r &'r Self, Vec<Relation<Self, $right_entity>>)>,
                crate::Error,
            > {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<
                    $right_entity,
                >>::get_entity_relations_as_batch_inner(conn, left_entities, true, false)
                .await
            }

            async fn delete_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<(), crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<
                    $right_entity,
                >>::delete_relations_inner(self, conn, true, false)
                .await
            }
        }
    };
}

macro_rules! impl_reverse_has_relation {
    ($left_entity: ty, $right_entity: ty) => {
        impl crate::models::musicbrainz::relations::traits::HasRelation<$left_entity>
            for $right_entity
        {
            const RELATION_TABLE: &str = const_format::formatcp!(
                "l_{}_{}",
                <$left_entity>::TABLE_NAME,
                <$right_entity>::TABLE_NAME
            );

            fn get_entity0_id(&self, other: &$left_entity) -> i64 {
                other.get_row_id()
            }

            fn get_entity1_id(&self, _other: &$left_entity) -> i64 {
                self.get_row_id()
            }

            async fn get_entity_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<Vec<Relation<Self, $left_entity>>, crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<$left_entity>>::get_entity_relations_inner(self, conn, false, true).await
            }

            async fn get_entity_relations_as_batch<'r>(
                conn: &mut sqlx::SqliteConnection,
                left_entities: &'r [&'r Self],
            ) -> Result<HashMap<i64, (&'r &'r Self, Vec<Relation<Self, $left_entity>>)>, crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<$left_entity>>::get_entity_relations_as_batch_inner(conn, left_entities, false, true).await
            }

            async fn delete_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<(), crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<
                    $right_entity,
                >>::delete_relations_inner(self, conn, false, true)
                .await
            }
        }
    };
}

use std::collections::HashMap;

pub(crate) use impl_reverse_has_relation;
use itertools::Itertools as _;
use self_relation::impl_has_self_relation;

use crate::RowId;
use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::genre::Genre;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::musicbrainz::url::Url;
use crate::models::musicbrainz::work::Work;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::sqlx_utils::entity_relations::JoinCollection;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

use super::Relation;

impl_has_self_relation!(Artist, Artist);
impl_has_relation!(Artist, Genre);
impl_reverse_has_relation!(Artist, Genre);
impl_has_relation!(Artist, Label);
impl_reverse_has_relation!(Artist, Label);
impl_has_relation!(Artist, Recording);
impl_reverse_has_relation!(Artist, Recording);
impl_has_relation!(Artist, Release);
impl_reverse_has_relation!(Artist, Release);
impl_has_relation!(Artist, ReleaseGroup);
impl_reverse_has_relation!(Artist, ReleaseGroup);
impl_has_relation!(Artist, Url);
impl_reverse_has_relation!(Artist, Url);
impl_has_relation!(Artist, Work);
impl_reverse_has_relation!(Artist, Work);

impl_has_self_relation!(Genre, Genre);
impl_has_relation!(Genre, Label);
impl_reverse_has_relation!(Genre, Label);
impl_has_relation!(Genre, Recording);
impl_reverse_has_relation!(Genre, Recording);
impl_has_relation!(Genre, Release);
impl_reverse_has_relation!(Genre, Release);
impl_has_relation!(Genre, ReleaseGroup);
impl_reverse_has_relation!(Genre, ReleaseGroup);
impl_has_relation!(Genre, Url);
impl_reverse_has_relation!(Genre, Url);
impl_has_relation!(Genre, Work);
impl_reverse_has_relation!(Genre, Work);

impl_has_self_relation!(Label, Label);
impl_has_relation!(Label, Recording);
impl_reverse_has_relation!(Label, Recording);
impl_has_relation!(Label, Release);
impl_reverse_has_relation!(Label, Release);
impl_has_relation!(Label, ReleaseGroup);
impl_reverse_has_relation!(Label, ReleaseGroup);
impl_has_relation!(Label, Url);
impl_reverse_has_relation!(Label, Url);
impl_has_relation!(Label, Work);
impl_reverse_has_relation!(Label, Work);

impl_has_self_relation!(Recording, Recording);
impl_has_relation!(Recording, Release);
impl_reverse_has_relation!(Recording, Release);
impl_has_relation!(Recording, ReleaseGroup);
impl_reverse_has_relation!(Recording, ReleaseGroup);
impl_has_relation!(Recording, Url);
impl_reverse_has_relation!(Recording, Url);
impl_has_relation!(Recording, Work);
impl_reverse_has_relation!(Recording, Work);

impl_has_self_relation!(Release, Release);
impl_has_relation!(Release, ReleaseGroup);
impl_reverse_has_relation!(Release, ReleaseGroup);
impl_has_relation!(Release, Url);
impl_reverse_has_relation!(Release, Url);
impl_has_relation!(Release, Work);
impl_reverse_has_relation!(Release, Work);

impl_has_self_relation!(ReleaseGroup, ReleaseGroup);
impl_has_relation!(ReleaseGroup, Url);
impl_reverse_has_relation!(ReleaseGroup, Url);
impl_has_relation!(ReleaseGroup, Work);
impl_reverse_has_relation!(ReleaseGroup, Work);

impl_has_self_relation!(Url, Url);
impl_has_relation!(Url, Work);
impl_reverse_has_relation!(Url, Work);

impl_has_self_relation!(Work, Work);

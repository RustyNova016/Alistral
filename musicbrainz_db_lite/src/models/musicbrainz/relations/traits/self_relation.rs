macro_rules! impl_has_self_relation {
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
                >>::get_entity_relations_inner(self, conn, true, true)
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
                >>::get_entity_relations_as_batch_inner(conn, left_entities, true, true)
                .await
            }

            async fn delete_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<(), crate::Error> {
                <Self as crate::models::musicbrainz::relations::traits::HasRelation<
                    $right_entity,
                >>::delete_relations_inner(self, conn, true, true)
                .await
            }
        }
    };
}

pub(crate) use impl_has_self_relation;

use crate::models::musicbrainz::relations::Relation;

impl<T> Relation<T, T> {
    /// Return the id of the other entity in the relationship
    pub fn get_other_id(&self, id: i64) -> i64 {
        if self.entity0 == id {
            self.entity1
        } else {
            self.entity0
        }
    }
}

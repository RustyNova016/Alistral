macro_rules! impl_inner_relation {
    ($right_entity: ty, $fn_name: ident) => {
        /// Get all the relations of the corresponding entity
        pub async fn $fn_name(
            &self,
            conn: &mut sqlx::SqliteConnection,
        ) -> Result<Vec<crate::models::musicbrainz::relations::Relation<Self, $right_entity>>, crate::Error> {
            <Self as crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>>::get_entity_relations(self, conn).await
        }
    };
}

macro_rules! impl_inner_delete_relations {
    ($right_entity: ty, $fn_name: ident) => {
        // Delete all the relations to the type of entity
        pub async fn $fn_name(
            &self,
            conn: &mut sqlx::SqliteConnection,
        ) -> Result<(), crate::Error> {
            <Self as crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>>::delete_relations(self, conn).await
        }
    };
}

macro_rules! impl_relations {
    ($left_entity: ty) => {
        impl $left_entity {
            // Getters

            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::artist::Artist,
                get_artist_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::genre::Genre,
                get_genre_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::label::Label,
                get_label_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::recording::Recording,
                get_recording_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::release::Release,
                get_release_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::release_group::ReleaseGroup,
                get_release_group_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::work::Work,
                get_work_relations
            );

            // Deletes

            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::artist::Artist,
                delete_artist_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::genre::Genre,
                delete_genre_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::label::Label,
                delete_label_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::recording::Recording,
                delete_recording_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::release::Release,
                delete_release_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::release_group::ReleaseGroup,
                delete_release_group_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_delete_relations!(
                crate::models::musicbrainz::work::Work,
                delete_work_relations
            );

            pub async fn delete_all_relations(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<(), crate::Error> {
                self.delete_artist_relations(conn).await?;
                self.delete_genre_relations(conn).await?;
                self.delete_label_relations(conn).await?;
                self.delete_recording_relations(conn).await?;
                self.delete_release_relations(conn).await?;
                self.delete_release_group_relations(conn).await?;
                self.delete_work_relations(conn).await?;
                Ok(())
            }
        }
    };
}

pub(crate) use impl_inner_delete_relations;
pub(crate) use impl_inner_relation;
pub(crate) use impl_relations;

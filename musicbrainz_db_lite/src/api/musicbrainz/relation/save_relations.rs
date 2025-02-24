macro_rules! impl_save_relation {
    ($left_entity: ty) => {
        impl $left_entity {
            pub(crate) async fn save_relation(
                &self,
                conn: &mut sqlx::SqliteConnection,
                api_relation: musicbrainz_rs_nova::entity::relations::Relation,
            ) -> Result<(), crate::Error> {
                Ok(match api_relation.content.clone() {
                    musicbrainz_rs_nova::entity::relations::RelationContent::Artist(value) => {
                        let entity1 = Artist::save_api_response(conn, *value).await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    // musicbrainz_rs does not support genre relations yet. TODO!
                    // musicbrainz_rs_nova::entity::relations::RelationContent::Genre(value) => {
                    //     let entity1 = crate::models::musicbrainz::genre::Genre::save_api_response(
                    //         conn, *value,
                    //     )
                    //     .await?;

                    //     crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                    //         conn,
                    //         api_relation,
                    //         self,
                    //         &entity1,
                    //     )
                    //     .await?;
                    // }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Label(value) => {
                        let entity1 = crate::models::musicbrainz::label::Label::save_api_response(
                            conn, *value,
                        )
                        .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Recording(value) => {
                        let entity1 = Recording::save_api_response(conn, *value).await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Release(value) => {
                        let entity1 =
                            crate::models::musicbrainz::release::Release::save_api_response(
                                conn, *value,
                            )
                            .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    },
                    musicbrainz_rs_nova::entity::relations::RelationContent::ReleaseGroup(value) => {
                        let entity1 =
                            crate::models::musicbrainz::release_group::ReleaseGroup::save_api_response(
                                conn, *value,
                            )
                            .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Work(value) => {
                        let entity1 =
                            crate::models::musicbrainz::work::Work::save_api_response(conn, *value)
                                .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    _ => Err(crate::Error::RelationNotImplemented)?,
                })
            }
        }
    };
}

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::musicbrainz::work::Work;
pub(crate) use impl_save_relation;

impl_save_relation!(Artist);
impl_save_relation!(Label);
impl_save_relation!(Recording);
impl_save_relation!(Release);
impl_save_relation!(ReleaseGroup);
impl_save_relation!(Work);

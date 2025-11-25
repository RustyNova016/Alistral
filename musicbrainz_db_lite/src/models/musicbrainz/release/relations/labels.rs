// use std::collections::HashMap;

// use itertools::Itertools as _;
// use sqlx::SqliteConnection;

use std::sync::Arc;

use crate::DBClient;
use crate::DBRelation;
use crate::Label;
// use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::release::Release;
// use crate::utils::sqlx_utils::entity_relations::JoinCollection;
// use crate::utils::sqlx_utils::entity_relations::JoinRelation;

/// Recording (1:M) -> Releases
pub struct ReleasesLabelsDBRel;

impl DBRelation<ReleasesLabelsDBRel> for Release {
    type ReturnedType = Label;

    fn get_join_statement() -> &'static str {
        "
        INNER JOIN labels_gid_redirect ON labels_gid_redirect.new_id = labels.id
        INNER JOIN label_infos ON label_infos.label = labels_gid_redirect.gid
        INNER JOIN releases ON releases.id = label_infos.release
        "
    }
}

impl Release {
    /// Get the labels associated to the release
    pub async fn get_labels_or_fetch(
        &self,
        client: &Arc<DBClient>,
    ) -> Result<Vec<Label>, crate::Error> {
        self.get_related_entity_or_fetch_as_task::<ReleasesLabelsDBRel>(client)
            .await
    }

    // pub async fn get_labels_or_fetch(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     client: &crate::DBClient,
    // ) -> Result<Vec<Label>, crate::Error> {
    //     // First, make sure all the data of the entity is in the database
    //     let id = self.get_or_fetch_as_complete(conn, client).await?.id;

    //     // Next, get all the children
    //     Ok(sqlx::query_as!(
    //         Label,
    //         "SELECT
    //                 labels.*
    //             FROM
    //                 releases
    //                 INNER JOIN label_infos ON releases.id = label_infos.release
    //                 INNER JOIN labels ON label_infos.label = labels_gid_redirect.gid
    //                 INNER JOIN labels_gid_redirect ON labels_gid_redirect.new_id = labels.id
    //             WHERE
    //                 releases.id = ?",
    //         id
    //     )
    //     .fetch_all(conn)
    //     .await?)
    // }

    // pub async fn get_labels_as_batch<'r>(
    //     conn: &mut sqlx::SqliteConnection,
    //     releases: &'r [&'r Release],
    // ) -> Result<HashMap<i64, (&'r &'r Release, Vec<Label>)>, crate::Error> {
    //     //#[cfg(debug_assertions)]
    //     //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
    //     let ids = releases.iter().map(|r| r.id).collect_vec();
    //     let id_string = serde_json::to_string(&ids)?;

    //     let joins: Vec<JoinRelation<i64, Label>> = sqlx::query_as(
    //         "
    //         SELECT
    //             releases.id as original_id,
    //             labels.*
    //         FROM
    //             releases
    //             INNER JOIN label_infos ON releases.id = label_infos.release
    //             INNER JOIN labels ON label_infos.label = labels_gid_redirect.gid
    //             INNER JOIN labels_gid_redirect ON labels_gid_redirect.new_id = labels.id
    //         WHERE
    //             releases.id IN (
    //                 SELECT
    //                     value
    //                 FROM
    //                     JSON_EACH(?)
    //             )
    //     ",
    //     )
    //     .bind(id_string)
    //     .fetch_all(conn)
    //     .await?;

    //     Ok(JoinCollection::from(joins).into_hashmap(releases, |id, value| &value.id == id))
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::database::client::DBClient;
//     use crate::database::pool::DBLitePoolExt as _;
//     use crate::models::musicbrainz::release::Release;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn should_get_labels_from_release() {
//         let client = DBClient::connect_in_memory_and_create().await.unwrap();
//         let conn = &mut *client.connection.get_raw_connection().await.unwrap();

//         // Test values. Feel free to add edge cases here
//         // (Release MBID, RG MBID)
//         let test_values = vec![(
//             "19d60a3e-0980-4ce9-bc3a-c72cb49ebd4c",
//             "ace30fb4-86c1-4dc6-b815-25e54f87f811",
//         )];

//         for (left, right) in test_values {
//             let value = Release::get_or_fetch(conn, &client, left)
//                 .await
//                 .expect("Error during fetch")
//                 .expect("The release should exists");

//             let right_value = value
//                 .get_labels_or_fetch(conn, &client)
//                 .await
//                 .expect("Error during fetching")
//                 .pop()
//                 .expect("The label should exists");

//             assert_eq!(right_value.mbid, right);

//             let right_value = Release::get_labels_as_batch(conn, &[&value])
//                 .await
//                 .expect("Error during fetching")
//                 .into_values()
//                 .map(|(_, mut labels)| labels.pop().expect("The label should exists"))
//                 .next()
//                 .expect("The label should exists");

//             assert_eq!(right_value.mbid, right);
//         }
//     }
// }

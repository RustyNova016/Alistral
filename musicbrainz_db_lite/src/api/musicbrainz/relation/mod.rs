pub mod save_relations;
use musicbrainz_rs::entity::relations::Relation as MBRelation;

use crate::RowId;
use crate::models::musicbrainz::relations::Relation;
use crate::models::musicbrainz::relations::traits::HasRelation;
use crate::utils::date_utils::date_string_to_timestamp;

impl<T, U> Relation<T, U>
where
    T: Send + Unpin + RowId + HasRelation<U>,
    U: Send + Unpin + RowId + HasRelation<T>,
{
    pub async fn save_api_response_inner(
        conn: &mut sqlx::SqliteConnection,
        value: MBRelation,
        fetched_entity: &T,
        content_entity: &U,
    ) -> Result<Relation<T, U>, crate::Error> {
        let relation = Relation {
            atribute_values: value
                .attribute_values
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            attribute_ids: value
                .attribute_ids
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            attributes: value
                .attributes
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            begin: value.begin.and_then(date_string_to_timestamp),
            direction: value.direction,
            end: value.end.and_then(date_string_to_timestamp),
            ended: value.ended.map(|end| if end { 1 } else { 0 }).unwrap_or(0),
            id: Default::default(),
            entity0: fetched_entity.get_entity0_id(content_entity),
            entity0_phamtom: Default::default(),
            entity1: fetched_entity.get_entity1_id(content_entity),
            entity1_phamtom: Default::default(),
            relation_type: value.relation_type,
            source_credit: value.source_credit,
            target_credit: value.target_credit,
            target_type: value.target_type,
            type_id: value.type_id,
        };

        Ok(relation.upsert(conn).await?)
    }
}

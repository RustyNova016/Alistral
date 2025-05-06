use crate::DBRelation;
use crate::Recording;
use crate::Url;
use crate::models::shared_traits::db_relation::EntityURLDBRel;

impl DBRelation<EntityURLDBRel> for Recording {
    type ReturnedType = Url;

    fn get_join_statement() -> &'static str {
        "INNER JOIN l_recordings_urls as rel ON rel.entity0 = recordings.id
        INNER JOIN urls ON urls.id = rel.entity1"
    }
}

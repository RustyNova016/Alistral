use crate::DBRelation;
use crate::Release;
use crate::Url;
use crate::models::shared_traits::db_relation::EntityActiveURLDBRel;

impl DBRelation<EntityActiveURLDBRel> for Release {
    type ReturnedType = Url;

    fn get_join_statement() -> &'static str {
        "INNER JOIN l_releases_urls as rel ON rel.entity0 = releases.id AND rel.end IS NULL AND rel.ended = 0
        INNER JOIN urls ON urls.id = rel.entity1"
    }
}

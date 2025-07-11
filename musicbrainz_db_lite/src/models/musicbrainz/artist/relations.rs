use crate::Artist;
use crate::DBRelation;
use crate::Url;
use crate::models::shared_traits::db_relation::EntityActiveURLDBRel;

impl DBRelation<EntityActiveURLDBRel> for Artist {
    type ReturnedType = Url;

    fn get_join_statement() -> &'static str {
        "INNER JOIN l_artists_urls as rel ON rel.entity0 = artists.id AND rel.end IS NULL AND rel.ended = 0
        INNER JOIN urls ON urls.id = rel.entity1"
    }
}

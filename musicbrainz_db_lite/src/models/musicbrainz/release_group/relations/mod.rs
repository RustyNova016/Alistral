use crate::ArtistCredit;
use crate::DBRelation;
use crate::ReleaseGroup;
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;

pub mod release;

impl DBRelation<ArtistCreditDBRel> for ReleaseGroup {
    type ReturnedType = ArtistCredit;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON release_groups.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit"
    }
}

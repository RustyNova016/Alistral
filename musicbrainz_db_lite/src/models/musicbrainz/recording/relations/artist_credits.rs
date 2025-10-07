use sequelles::JoinCollection;
use tuillez::tracing_utils::pg_future::PGFuture as _;

use crate::Artist;
use crate::DBRelation;
use crate::Recording;
use crate::models::shared_traits::db_relation::ArtistFromCreditsRelation;

impl Recording {
    pub async fn get_artists_from_credit_relation_bulk(
        &self,
        conn: &mut sqlx::SqliteConnection,
        recording_refs: &[&Self],
    ) -> Result<JoinCollection<Artist>, crate::Error> {
        Recording::get_related_entity_bulk::<ArtistFromCreditsRelation>(conn, recording_refs)
            .pg_spinner("Loading recordings from cache...")
            .await
    }
}

impl DBRelation<ArtistFromCreditsRelation> for Recording {
    type ReturnedType = Artist;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON recordings.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
        INNER JOIN artists_gid_redirect ON artist_credits_item.artist_gid = artists_gid_redirect.gid
        INNER JOIN artists ON artists_gid_redirect.new_id = artists.id"
    }
}

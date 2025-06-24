use crate::Artist;
use crate::ArtistCredit;
use crate::DBRelation;
use crate::Recording;
use crate::Release;
use crate::Track;
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;
use crate::models::shared_traits::db_relation::ArtistFromCreditsRelation;

/// [`crate::Track`] (N:1) -> [`crate::Release`]
pub struct TrackReleaseDBRel;

impl DBRelation<TrackReleaseDBRel> for Track {
    type ReturnedType = Release;

    fn get_join_statement() -> &'static str {
        "INNER JOIN medias ON tracks.media = medias.id
        INNER JOIN releases ON medias.`release` = releases.id"
    }
}

/// [`crate::Track`] (1:1) -> [`crate::Recording`]
pub struct TrackRecordingDBRel;

impl DBRelation<TrackRecordingDBRel> for Track {
    type ReturnedType = Recording;

    fn get_join_statement() -> &'static str {
        "INNER JOIN recordings ON recordings.id = tracks.recording"
    }
}

impl DBRelation<ArtistFromCreditsRelation> for Track {
    type ReturnedType = Artist;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON tracks.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
        INNER JOIN artists_gid_redirect ON artist_credits_item.artist_gid = artists_gid_redirect.gid
        INNER JOIN artists ON artists_gid_redirect.new_id = artists.id"
    }
}

impl DBRelation<ArtistCreditDBRel> for Track {
    type ReturnedType = ArtistCredit;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON tracks.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit"
    }
}

impl Track {
    /// Get the release associated with this track
    pub async fn get_release(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<Release>, crate::Error> {
        Ok(self
            .get_related_entity::<TrackReleaseDBRel>(conn)
            .await?
            .into_iter()
            .next())
    }

    /// Get the recording associated with this track
    pub async fn get_recording(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<Recording>, crate::Error> {
        Ok(self
            .get_related_entity::<TrackRecordingDBRel>(conn)
            .await?
            .into_iter()
            .next())
    }
}

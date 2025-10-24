use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_score::TopScore;

pub trait TopEntity<Sco>
where
    Self: Sized + HasRowID,
    Sco: TopScorer<Self>,
{
    fn get_top_score(&self, scorer: Sco) -> TopScore<Sco::Score> {
        scorer.get_score_of_element(&self)
    }

    fn get_display_entity(
        self,
    ) -> impl FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>;
}

impl<Sco, Ent, Lis> TopEntity<Sco> for EntityWithListens<Ent, Lis>
where
    Sco: TopScorer<Self>,
    Ent: HasRowID + Clone,
    Ent: FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
    Lis: ListenCollectionReadable,
{
    fn get_display_entity(
        self,
    ) -> impl FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error> {
        self.into_entity()
    }
}

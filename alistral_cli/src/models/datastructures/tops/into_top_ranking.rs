use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use sequelles::datastructures::ranking::Ranking;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_entity::TopEntity;

/// Trait for anything that can tunr into a top ranking
pub trait IntoTopRanking<Ent, Sco, Map>
where
    Self: Sized,
    Ent: TopEntity<Sco> + 'static,
    Sco: TopScorer<Ent> + 'static,
{
    fn get_entities(self, mapping: &Map) -> Vec<Ent>;

    fn into_top_ranking(self, scorer: &Sco, mapping: &Map) -> Vec<(usize, Ent)> {
        let entities = self.get_entities(mapping);

        let ranking = Ranking::from(entities);
        ranking.get_ranks(|ent| scorer.get_score_of_element(ent))
    }
}

impl<Ent, Lis, Sco> IntoTopRanking<EntityWithListens<Ent, Lis>, Sco, Vec<Ent>>
    for EntityWithListensCollection<Ent, Lis>
where
    Sco: TopScorer<EntityWithListens<Ent, Lis>> + 'static,
    Ent: HasRowID
        + Clone
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>
        + 'static,
    Lis: ListenCollectionReadable + Default + 'static,
    EntityWithListens<Ent, Lis>: Clone,
{
    fn get_entities(self, mapping: &Vec<Ent>) -> Vec<EntityWithListens<Ent, Lis>> {
        mapping
            .iter()
            .map(|ent| match self.get_by_id(ent.rowid()) {
                Some(stat) => stat.to_owned(),
                None => EntityWithListens::new(Ent::clone(ent), Default::default()),
            })
            .collect_vec()
    }
}

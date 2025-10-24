use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use musicbrainz_db_lite::HasRowID;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::datastructures::tops::generator::TopGenerator;
use crate::models::datastructures::tops::into_top_ranking::IntoTopRanking as _;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::scorer::TopScorer;

impl TopGenerator {
    pub fn generate_rows_for_entwlis<Ent, Lis, Sco>(
        current: EntityWithListensCollection<Ent, Lis>,
        previous: Option<EntityWithListensCollection<Ent, Lis>>,
        scorer: Sco,
    ) -> Vec<TopRow>
    where
        Sco: TopScorer<EntityWithListens<Ent, Lis>> + 'static,
        Ent: HasRowID
            + Clone
            + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>
            + 'static,
        Lis: ListenCollectionReadable + Default + 'static,
        EntityWithListens<Ent, Lis>: Clone,
    {
        let entities = current
            .iter_entities()
            .chain(previous.iter().flat_map(|stats| stats.iter_entities()))
            .unique_by(|ent| ent.rowid())
            .cloned()
            .collect_vec();

        let current = current.into_top_ranking(&scorer, &entities);
        let previous = previous.map(|previous| previous.into_top_ranking(&scorer, &entities));

        Self::generate_top_rows(current, previous, &scorer)
    }
}

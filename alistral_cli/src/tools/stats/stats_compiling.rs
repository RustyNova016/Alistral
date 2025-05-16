use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::tags::TagWithEntListensCollection;
use alistral_core::datastructures::entity_with_listens::tags::TagWithEntListensStrategy;
use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithRecordingsStrategy;
use alistral_core::datastructures::listen_collection::ListenCollection;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::recording_strategy;
use crate::tools::stats::StatsCommand;

impl StatsCommand {
    pub(super) fn tag_strategy(
        &self,
    ) -> TagWithEntListensStrategy<
        '_,
        RecordingWithListenStrategy<'_>,
        musicbrainz_db_lite::models::musicbrainz::recording::Recording,
        ListenCollection,
    > {
        TagWithEntListensStrategy::new(&ALISTRAL_CLIENT.core, recording_strategy(&ALISTRAL_CLIENT))
    }

    pub(super) async fn tag_stats(
        &self,
        user: String,
    ) -> Result<TagWithEntListensCollection<Recording, ListenCollection>, crate::Error> {
        Ok(ListenFetchQuery::get_entity_with_listens(
            &ALISTRAL_CLIENT.core,
            user,
            &self.tag_strategy(),
        )
        .await?)
    }

    pub(super) fn work_strategy(&self) -> WorkWithRecordingsStrategy {
        let mut strat = WorkWithRecordingsStrategy::new(
            &ALISTRAL_CLIENT.core,
            recording_strategy(&ALISTRAL_CLIENT),
        );

        if self.w_recursive {
            strat = strat.with_recursive_parents()
        }

        strat
    }

    pub(super) async fn work_stats(
        &self,
        user: String,
    ) -> Result<WorkWithRecordingsCollection, crate::Error> {
        Ok(ListenFetchQuery::get_entity_with_listens(
            &ALISTRAL_CLIENT.core,
            user,
            &self.work_strategy(),
        )
        .await?)
    }
}

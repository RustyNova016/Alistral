use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use convert_case::Casing as _;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::cells::entity_name_cell::EntityNameCell;
use crate::models::cli_components::tables::cells::listen_count_cell::ListenCountCell;
use crate::models::cli_components::tables::cells::listen_duration_cell::ListenDurationCell;
use crate::models::cli_components::tables::cells::rank_cell::RankCell;
use crate::models::cli_components::tables::cells::top_cell::TopCell;
use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::TableRow;

pub struct TopListenDurCountRow<Ent> {
    pub entity_name: EntityNameCell<Ent>,

    pub listen_duration: ListenDurationCell,
    pub listen_counts: ListenCountCell,
}

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for TopListenDurCountRow<Ent>
where
    Ent: Clone,
    EntityWithListens<Ent, Lis>: ListenCollWithTime + ListenCollectionReadable,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        Self {
            entity_name: EntityNameCell {
                entity: value.entity().clone(),
            },
            listen_duration: ListenDurationCell(TopCell::new(
                Some(value.get_time_listened().into()),
                None,
            )),
            listen_counts: ListenCountCell(TopCell::new(Some(value.listen_count()), None)),
        }
    }
}

impl<Ent, Lis> From<EntityListensComparison<Ent, Lis>> for TopListenDurCountRow<Ent>
where
    Ent: Clone,
    Lis: Default,
    EntityWithListens<Ent, Lis>: ListenCollWithTime + Clone + ListenCollectionReadable,
{
    fn from(value: EntityListensComparison<Ent, Lis>) -> Self {
        Self {
            entity_name: EntityNameCell {
                entity: value.entity().unwrap().clone(),
            },

            listen_duration: ListenDurationCell(TopCell::new(
                value
                    .current_or_empty()
                    .map(|cur| cur.get_time_listened().into()),
                value
                    .previous_or_empty()
                    .map(|prev| prev.get_time_listened().into()),
            )),

            listen_counts: ListenCountCell(TopCell::new(
                value.current_or_empty().map(|cur| cur.listen_count()),
                value.previous_or_empty().map(|prev| prev.listen_count()),
            )),
        }
    }
}

impl<Ent> TableRow for TopListenDurCountRow<Ent>
where
    Ent: MusicbrainzEntity
        + HasRowID
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
{
    fn get_table_header() -> Vec<String> {
        vec![
            "Rank".to_string(),
            "Listen count".to_string(),
            "Listen duration".to_string(),
            Ent::entity_name().to_case(convert_case::Case::Sentence),
        ]
    }

    async fn format(&self, ranking: RankCell, show_prev: bool) -> comfy_table::Row {
        vec![
            ranking.format(4, 4, show_prev),
            self.listen_counts.format(4, 4, show_prev),
            self.listen_duration.format(4, 4, show_prev),
            self.entity_name.format().await,
        ]
        .into()
    }

    fn get_row_id(&self) -> i64 {
        self.entity_name.entity.rowid()
    }
}

impl<Ent> TableSort<TopListenDurCountRow<Ent>> for OrderTableByListenDuration {
    fn get_row_score(&self, row: &TopListenDurCountRow<Ent>) -> impl Ord {
        row.listen_duration.0.current.clone().unwrap_or_default()
    }

    fn get_row_prev_score(&self, row: &TopListenDurCountRow<Ent>) -> Option<impl Ord> {
        row.listen_duration.0.previous.as_ref()
    }
}

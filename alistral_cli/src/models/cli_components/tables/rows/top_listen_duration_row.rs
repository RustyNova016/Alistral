use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use convert_case::Casing as _;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::cli_components::tables::cells::entity_name_cell::EntityNameCell;
use crate::models::cli_components::tables::cells::listen_duration_cell::ListenDurationCell;
use crate::models::cli_components::tables::cells::rank_cell::RankCell;
use crate::models::cli_components::tables::cells::top_cell::TopCell;
use crate::models::cli_components::tables::rows::TableRow;

pub struct TopListenDurationRow<Ent> {
    pub entity_name: EntityNameCell<Ent>,

    pub listen_duration: ListenDurationCell,
}

impl<Ent> TopListenDurationRow<Ent> {
    pub fn from_current_previous_entity_with_listen<Lis>(
        cur: Option<EntityWithListens<Ent, Lis>>,
        prev: Option<EntityWithListens<Ent, Lis>>,
    ) -> Self
    where
        EntityWithListens<Ent, Lis>: ListenCollWithTime,
        Ent: HasRowID + Clone,
        Lis: ListenCollectionReadable,
    {
        Self {
            entity_name: EntityNameCell {
                entity: cur
                    .as_ref()
                    .map(|cur| cur.entity())
                    .or_else(|| prev.as_ref().map(|prev| prev.entity()))
                    .cloned()
                    .unwrap(),
            },
            listen_duration: ListenDurationCell(TopCell::new(
                cur.map(|cur| cur.get_time_listened().into()),
                prev.map(|prev| prev.get_time_listened().into()),
            )),
        }
    }
}

impl<Ent, Lis> From<EntityListensComparison<Ent, Lis>> for TopListenDurationRow<Ent>
where
    Ent: Clone,
    Lis: Default,
    EntityWithListens<Ent, Lis>: ListenCollWithTime + Clone,
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
        }
    }
}

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for TopListenDurationRow<Ent>
where
    EntityWithListens<Ent, Lis>: ListenCollWithTime,
    Ent: HasRowID + Clone,
    Lis: ListenCollectionReadable,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        Self::from_current_previous_entity_with_listen(Some(value), None)
    }
}

impl<Ent> TableRow for TopListenDurationRow<Ent>
where
    Ent: MusicbrainzEntity
        + HasRowID
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
{
    fn get_table_header() -> Vec<String> {
        vec![
            "Rank".to_string(),
            "Listen duration".to_string(),
            Ent::entity_name().to_case(convert_case::Case::Sentence),
        ]
    }

    async fn format(&self, ranking: RankCell, show_prev: bool) -> comfy_table::Row {
        vec![
            ranking.format(4, 4, show_prev),
            self.listen_duration.format(4, 4, show_prev),
            self.entity_name.format().await,
        ]
        .into()
    }

    fn get_row_id(&self) -> i64 {
        self.entity_name.entity.rowid()
    }
}

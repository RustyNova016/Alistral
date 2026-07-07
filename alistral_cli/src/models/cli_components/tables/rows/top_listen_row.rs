use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use convert_case::Casing as _;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::cli_components::tables::cells::entity_name_cell::EntityNameCell;
use crate::models::cli_components::tables::cells::listen_count_cell::ListenCountCell;
use crate::models::cli_components::tables::cells::rank_cell::RankCell;
use crate::models::cli_components::tables::cells::top_cell::TopCell;
use crate::models::cli_components::tables::rows::TableRow;

pub struct TopListenCountsRow<Ent> {
    pub entity_name: EntityNameCell<Ent>,

    pub listen_counts: ListenCountCell,
    //pub rank: RankCell,
}

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for TopListenCountsRow<Ent>
where
    Ent: HasRowID + Clone,
    Lis: ListenCollectionReadable,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        Self {
            entity_name: EntityNameCell {
                entity: value.entity().clone(),
            },
            listen_counts: ListenCountCell(TopCell::new(Some(value.listen_count()), None)),
            //rank: RankCell(0)
        }
    }
}

impl<Ent> TableRow for TopListenCountsRow<Ent>
where
    Ent: MusicbrainzEntity
        + HasRowID
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
{
    fn get_table_header() -> Vec<String> {
        vec![
            "Rank".to_string(),
            "Listen count".to_string(),
            Ent::entity_name().to_case(convert_case::Case::Sentence),
        ]
    }

    async fn format(&self, ranking: RankCell, show_prev: bool) -> comfy_table::Row {
        vec![
            ranking.format(4, 4, show_prev),
            self.listen_counts.format(4, 4, show_prev),
            self.entity_name.format().await,
        ]
        .into()
    }

    fn get_row_id(&self) -> i64 {
        self.entity_name.entity.rowid()
    }
}

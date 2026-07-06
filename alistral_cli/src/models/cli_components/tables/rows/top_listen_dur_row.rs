use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use convert_case::Casing;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::models::cli_components::tables::cells::entity_name_cell::EntityNameCell;
use crate::models::cli_components::tables::cells::listen_duration_cell::ListenDurationCell;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::datastructures::tops::printer::top_cell::TopCell;

pub struct TopListenDurCountsRow<Ent> {
    pub entity_name: EntityNameCell<Ent>,

    pub listen_duration: ListenDurationCell,
}

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for TopListenDurCountsRow<Ent>
where
    EntityWithListens<Ent, Lis>: ListenCollWithTime,
    Ent: HasRowID + Clone,
    Lis: ListenCollectionReadable,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        Self {
            entity_name: EntityNameCell {
                entity: value.entity().clone(),
            },
            listen_duration: ListenDurationCell(TopCell::new(Some(value.get_time_listened().into()), None, false)),
        }
    }
}

impl<Ent> TableRow for TopListenDurCountsRow<Ent>
where
    Ent: MusicbrainzEntity
        + FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
{
    fn get_table_header() -> Vec<String> {
        vec![
            "Rank".to_string(),
            "Listen duration".to_string(),
            Ent::entity_name().to_case(convert_case::Case::Sentence),
        ]
    }

    async fn format(&self, ranking: usize) -> comfy_table::Row {
        vec![
            ranking.to_string(),
            self.listen_duration.0.format(4, 4),
            self.entity_name.format().await,
        ]
        .into()
    }
}

// impl<Ent> OrderWith<OrderTableByListenCount> for Vec<TopListenCountsRow<Ent>> {
//     fn order_with_mut(&mut self, orderer: OrderTableByListenCount) -> &mut Self {
//         self.sort_unstable_by_key(|row| row.listen_counts.0.current);

//         if orderer.desc {
//             self.reverse();
//         }

//         self
//     }
// }

use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::entity_comparison::collection::EntityListensComparisonCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::traits::sorter::InsertElement as _;
use itertools::Itertools as _;
use musicbrainz_db_lite::Recording;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::tables::order_by::OrderTableByListenCount;
use crate::models::cli_components::tables::rows::top_listen_row::TopListenCountsRow;
use crate::models::cli_components::tables::table::TopTable;

pub(super) async fn top_next_recordings(
    data: &EntityListensComparison<Recording, ListenCollection>,
    user_listens: &ListenCollection,
    show_previous: bool,
) -> String {
    let (curr_listens, prev_listens) = data.map_as(|data| {
        data.iter_listens()
            .filter_map(|listen| user_listens.get_next_listen(listen))
            .cloned()
            .collect_vec()
    });

    let current = match curr_listens {
        Some(val) => {
            let mut coll = RecordingWithListensCollection::new();
            coll.insert_elements(ALISTRAL_CLIENT.core.as_ref(), val)
                .await
                .unwrap();
            coll
        }
        None => EntityWithListensCollection::default(),
    };

    let previous = match prev_listens {
        Some(val) => {
            let mut coll = RecordingWithListensCollection::new();
            coll.insert_elements(ALISTRAL_CLIENT.core.as_ref(), val)
                .await
                .unwrap();
            coll
        }
        None => EntityWithListensCollection::default(),
    };

    let mut comp = EntityListensComparisonCollection::default();

    comp.insert_current_iter(current);
    comp.insert_previous_iter(previous);

    let mut table = TopTable::<TopListenCountsRow<_>, _>::from_entity_listens_comps(
        comp,
        OrderTableByListenCount,
        true,
    );
    table.set_show_previous(show_previous);

    table.format(20, 0).await
}

use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::rows::top_listen_dur_row::TopListenDurCountsRow;
use crate::models::cli_components::tables::rows::top_listen_row::TopListenCountsRow;

#[derive(Debug, Default)]
pub struct OrderTableByListenCount;

impl<Ent> TableSort<TopListenCountsRow<Ent>> for OrderTableByListenCount {
    fn get_row_score(&self, row: &TopListenCountsRow<Ent>) -> impl Ord {
        row.listen_counts.0.current.unwrap_or_default()
    }
}

#[derive(Debug, Default)]
pub struct OrderTableByListenDuration;

impl<Ent> TableSort<TopListenDurCountsRow<Ent>> for OrderTableByListenDuration {
    fn get_row_score(&self, row: &TopListenDurCountsRow<Ent>) -> impl Ord {
        row.listen_duration.0.current.clone().unwrap_or_default()
    }
}

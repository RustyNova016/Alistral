use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::rows::top_listen_duration_row::TopListenDurationRow;
use crate::models::cli_components::tables::rows::top_listen_row::TopListenCountsRow;

#[derive(Debug, Default)]
pub struct OrderTableByListenCount;

impl<Ent> TableSort<TopListenCountsRow<Ent>> for OrderTableByListenCount {
    fn get_row_score(&self, row: &TopListenCountsRow<Ent>) -> impl Ord {
        row.listen_counts.0.current.unwrap_or_default()
    }

    fn get_row_prev_score(&self, row: &TopListenCountsRow<Ent>) -> Option<impl Ord> {
        row.listen_counts.0.previous.as_ref()
    }
}

#[derive(Debug, Default)]
pub struct OrderTableByListenDuration;

impl<Ent> TableSort<TopListenDurationRow<Ent>> for OrderTableByListenDuration {
    fn get_row_score(&self, row: &TopListenDurationRow<Ent>) -> impl Ord {
        row.listen_duration.0.current.clone().unwrap_or_default()
    }

    fn get_row_prev_score(&self, row: &TopListenDurationRow<Ent>) -> Option<impl Ord> {
        row.listen_duration.0.previous.as_ref()
    }
}

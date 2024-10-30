use core::cmp::Reverse;

use rust_decimal::Decimal;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::config::Config;

pub fn overdue_sorter(mut recordings: Vec<RecordingWithListens>) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_by().num_seconds();
        Decimal::from(score) * conf.bumps.get_multiplier2(&r.recording().mbid)
    });

    recordings
}

pub fn overdue_factor_sorter(
    mut recordings: Vec<RecordingWithListens>,
) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_factor() + Decimal::ONE;
        if r.recording().mbid == "28096171-7be9-4756-9018-3506eab8fbd4" {
            println!("Overdue score {}", score * conf.bumps.get_multiplier2(&r.recording().mbid))
        }
        Reverse(score * conf.bumps.get_multiplier2(&r.recording().mbid))
    });

    recordings
}

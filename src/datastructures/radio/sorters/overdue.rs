use core::cmp::Reverse;

use async_fn_stream::fn_stream;
use chrono::Duration;
use chrono::Utc;
use futures::Stream;
use rust_decimal::Decimal;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::config::Config;

pub fn overdue_sorter(mut recordings: Vec<RecordingWithListens>) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_by().num_seconds();
        Decimal::from(score)
            * conf
                .read_or_panic()
                .bumps
                .get_multiplier(&r.recording().mbid)
    });

    recordings
}

pub fn overdue_factor_sorter(
    mut recordings: Vec<RecordingWithListens>,
) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_factor() + Decimal::ONE;
        Reverse(
            score
                * conf
                    .read_or_panic()
                    .bumps
                    .get_multiplier(&r.recording().mbid),
        )
    });

    recordings
}

pub fn overdue_factor_sorter_cumulative(
    mut recordings: Vec<RecordingWithListens>,
) -> impl Stream<Item = RecordingWithListens> {
    let conf = Config::load_or_panic();

    fn_stream(|emitter| async move {
        let mut curr_time = Utc::now();
        while !recordings.is_empty() {
            let top_recording = recordings
                .iter()
                .enumerate()
                .max_by_key(|r| {
                    let score = r.1.overdue_factor_at(&curr_time) + Decimal::ONE;
                    score
                        * conf
                            .read_or_panic()
                            .bumps
                            .get_multiplier(&r.1.recording().mbid)
                })
                .expect("There should be at least one recording");

            let top_recording = recordings.remove(top_recording.0);

            curr_time += top_recording
                .recording()
                .length_as_duration()
                .unwrap_or(Duration::zero());

            emitter.emit(top_recording).await;
        }
    })
}

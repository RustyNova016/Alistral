use core::cmp::Reverse;

use alistral_core::cli::progress_bar::ProgressBarCli;

use crate::datastructures::entity_with_listens::recording_with_listens::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::data::listenbrainz::popularity::PopularityRecordingResponseItem;

/// Sort listens based on the rate of listens of a recording
pub fn underrated_sorter(
    mut recordings: Vec<RecordingWithListens>,
    user_listens: &RecordingWithListensCollection,
    global_listen_counts: Vec<PopularityRecordingResponseItem>,
) -> Vec<RecordingWithListens> {
    let progress = ProgressBarCli::new((recordings.len()) as u64, Some("Sorting recordings"));
    recordings.sort_by_cached_key(|r| {
        let global_count = global_listen_counts
            .iter()
            .find_map(|c| {
                if c.recording_mbid == r.recording().mbid {
                    return c.total_listen_count;
                }
                None
            })
            .unwrap_or(0);

        let score = r.get_underated_score(user_listens, global_count);

        progress.inc(1);

        Reverse(score)
    });

    recordings
}

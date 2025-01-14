use core::cmp::Reverse;

use crate::datastructures::entity_with_listens::recording_with_listens::collection::RecordingWithListensCollectionOld;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListensOld;
use crate::models::data::listenbrainz::popularity::PopularityRecordingResponseItem;
use crate::utils::cli::progress_bar::ProgressBarCli;

/// Sort listens based on the rate of listens of a recording
pub fn underrated_sorter(
    mut recordings: Vec<RecordingWithListensOld>,
    user_listens: &RecordingWithListensCollectionOld,
    global_listen_counts: Vec<PopularityRecordingResponseItem>,
) -> Vec<RecordingWithListensOld> {
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

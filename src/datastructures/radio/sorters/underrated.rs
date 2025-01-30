use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::models::data::listenbrainz::popularity::PopularityRecordingResponseItem;

/// Sort listens based on the rate of listens of a recording
#[instrument( fields(indicatif.pb_show = tracing::field::Empty))]
pub fn underrated_sorter(
    mut recordings: Vec<RecordingWithListens>,
    user_listens: &RecordingWithListensCollection,
    global_listen_counts: Vec<PopularityRecordingResponseItem>,
) -> Vec<RecordingWithListens> {
    pg_counted!(recordings.len(), "Sorting recordings");

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

        pg_inc!();

        Reverse(score)
    });

    recordings
}

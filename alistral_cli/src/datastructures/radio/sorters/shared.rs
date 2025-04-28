use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use futures::StreamExt;

pub async fn shared_listens_sorter(
    radio: impl StreamExt<Item = RecordingWithListens>,
    other: Vec<RecordingWithListens>,
) -> Vec<RecordingWithListens> {
    let mut radio = radio.collect::<Vec<_>>().await;

    radio.sort_by_cached_key(|track| {
        let other_recording_listens = other
            .iter()
            .find(|rec| rec.recording().mbid == track.recording().mbid)
            .map(|rec| rec.listen_count())
            .unwrap_or(0);

        Reverse(track.listen_count().min(other_recording_listens))
    });

    radio
}

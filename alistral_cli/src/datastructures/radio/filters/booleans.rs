use core::future::ready;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use futures::StreamExt;

pub fn and_filter(
    radio: impl StreamExt<Item = RecordingWithListens>,
    other: Vec<RecordingWithListens>,
) -> impl StreamExt<Item = RecordingWithListens> {
    radio.filter(move |track| {
        ready(
            other
                .iter()
                .any(|other_track| track.recording().mbid == other_track.recording().mbid),
        )
    })
}

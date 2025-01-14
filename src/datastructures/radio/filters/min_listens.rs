use core::future::ready;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use futures::{Stream, StreamExt};

pub fn min_listen_filter(
    listens: impl StreamExt<Item = RecordingWithListens>,
    minimum_listens: u64,
) -> impl Stream<Item = RecordingWithListens> {
    listens.filter(move |r| ready(r.listen_count() as u64 >= minimum_listens))
}

use core::future::ready;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::{Duration, Utc};
use futures::{Stream, StreamExt};

pub fn cooldown_filter(
    recordings: impl StreamExt<Item = RecordingWithListens>,
    cooldown: Duration,
) -> impl Stream<Item = RecordingWithListens> {
    recordings.filter(move |r| {
        let Some(last_listen_date) = r.latest_listen_date() else {
            return ready(true);
        };

        let after_cooldown = last_listen_date + cooldown;

        ready(after_cooldown <= Utc::now())
    })
}

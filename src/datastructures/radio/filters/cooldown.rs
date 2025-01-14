use core::future::ready;

use chrono::{Duration, Utc};
use futures::{Stream, StreamExt};

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListensOld;

pub fn cooldown_filter(
    recordings: impl StreamExt<Item = RecordingWithListensOld>,
    cooldown: Duration,
) -> impl Stream<Item = RecordingWithListensOld> {
    recordings.filter(move |r| {
        let Some(last_listen_date) = r.last_listen_date() else {
            return ready(true);
        };

        let after_cooldown = last_listen_date + cooldown;

        ready(after_cooldown <= Utc::now())
    })
}

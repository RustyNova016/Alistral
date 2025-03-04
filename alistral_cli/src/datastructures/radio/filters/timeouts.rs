use core::future::ready;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use futures::{Stream, StreamExt};

use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

pub fn timeout_filter(
    listens: impl StreamExt<Item = RecordingWithListens>,
) -> impl Stream<Item = RecordingWithListens> {
    let config = RecordingTimeoutConfig::load().expect("Couldn't fetch the timeout config");
    let config = config.read_or_panic();
    let timeouts = config.get_timed_out_recordings();

    listens.filter(move |r| ready(!timeouts.iter().any(|t| *t == r.recording().mbid)))
}

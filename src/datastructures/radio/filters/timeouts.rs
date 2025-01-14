use core::future::ready;

use futures::{Stream, StreamExt};

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListensOld;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

pub fn timeout_filter(
    listens: impl StreamExt<Item = RecordingWithListensOld>,
) -> impl Stream<Item = RecordingWithListensOld> {
    let config = RecordingTimeoutConfig::load().expect("Couldn't fetch the timeout config");
    let config = config.read_or_panic();
    let timeouts = config.get_timed_out_recordings();

    listens.filter(move |r| ready(!timeouts.iter().any(|t| *t == r.recording().mbid)))
}

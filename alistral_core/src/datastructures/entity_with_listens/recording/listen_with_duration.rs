use chrono::TimeDelta;

use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::models::listen_with_time::ListenWithDuration;
use crate::models::listen_with_time::iterator::IntoListenWithDurationIterator;

impl IntoListenWithDurationIterator for RecordingWithListens {
    fn into_listen_with_duration_iterator(self) -> impl Iterator<Item = ListenWithDuration> {
        self.listens.into_iter().map(move |listen| {
            ListenWithDuration::new(
                listen,
                self.entity.length_as_duration().unwrap_or_else(|| {
                    // warn!(
                    //     "Recording `{}` ({}) is missing its duration in MusicBrainz.",
                    //     self.entity.mbid, self.entity.title
                    // );
                    TimeDelta::zero()
                }),
            )
        })
    }
}


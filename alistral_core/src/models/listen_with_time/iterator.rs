use chrono::TimeDelta;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;

use crate::models::listen_with_time::ListenWithDuration;

/// Iterate over [`ListenWithDuration`]
pub trait IntoListenWithDurationIterator: Sized {
    fn into_listen_with_duration_iterator(self) -> impl Iterator<Item = ListenWithDuration>;

    /// Returns the total duration of all the listens of the iterator
    fn total_listened_duration(self) -> TimeDelta {
        self.into_listen_with_duration_iterator()
            .unique_by(|l| l.rowid())
            .map(|l| l.duration)
            .sum()
    }
}

impl<T> IntoListenWithDurationIterator for T
where
    T: Iterator<Item = ListenWithDuration>,
{
    fn into_listen_with_duration_iterator(self) -> impl Iterator<Item = ListenWithDuration> {
        self
    }
}

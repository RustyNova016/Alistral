use chrono::TimeDelta;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;

use crate::models::listen_with_time::ListenWithDuration;

/// Iterate over [`&ListenWithDuration`](ListenWithDuration)
pub trait AsListenWithDurationIterator: Sized {
    fn as_listen_with_duration_iterator(&self) -> impl Iterator<Item = &ListenWithDuration>;

    /// Returns the total duration of all the listens of the iterator
    fn total_listened_duration(self) -> TimeDelta {
        self.as_listen_with_duration_iterator()
            .unique_by(|l| l.rowid())
            .map(|l| l.duration)
            .sum()
    }
}

impl<'s> AsListenWithDurationIterator for &'s [ListenWithDuration] {
    fn as_listen_with_duration_iterator(&self) -> impl Iterator<Item = &ListenWithDuration> {
        self.iter()
    }
}

impl AsListenWithDurationIterator for Vec<ListenWithDuration> {
    fn as_listen_with_duration_iterator(&self) -> impl Iterator<Item = &ListenWithDuration> {
        self.iter()
    }
}

impl<'l> AsListenWithDurationIterator for Vec<&'l ListenWithDuration> {
    fn as_listen_with_duration_iterator(&self) -> impl Iterator<Item = &ListenWithDuration> {
        self.iter().map(|l| *l)
    }
}

pub mod ref_iterator;
use chrono::TimeDelta;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

pub mod iterator;

/// A Listen with an associated duration.
///
/// This duration isn't stored in the listen data, but actually the recording's data. So this structs allow decoupling the duration from the recording.
pub struct ListenWithDuration {
    pub listen: Listen,
    pub duration: TimeDelta,
}

impl ListenWithDuration {
    pub fn new(listen: Listen, duration: TimeDelta) -> Self {
        Self { listen, duration }
    }
}

impl HasRowID for ListenWithDuration {
    fn rowid(&self) -> i64 {
        self.listen.id
    }
}

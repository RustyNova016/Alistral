use chrono::TimeDelta;

use crate::models::listenbrainz::listen::Listen;

impl Listen {
    pub fn duration_as_timedelta(&self) -> Option<TimeDelta> {
        self.duration.and_then(|dur| TimeDelta::new(dur, 0))
    }
}

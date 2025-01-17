use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

pub struct TimeWindow {
    duration: Duration,
    start: DateTime<Utc>,
}

impl TimeWindow {
    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.start + self.duration
    }

    pub fn previous_start(&self) -> DateTime<Utc> {
        self.start - self.duration
    }
}

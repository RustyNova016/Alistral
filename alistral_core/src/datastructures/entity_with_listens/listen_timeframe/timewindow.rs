use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;

pub struct TimeWindow {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl TimeWindow {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn from_duration(duration: Duration, start: DateTime<Utc>) -> Self {
        Self {
            end: start + duration,
            start,
        }
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn previous_start(&self) -> DateTime<Utc> {
        self.start - self.duration()
    }
}

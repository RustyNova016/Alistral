use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct TimeFrame {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeFrame {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn duration(&self) -> TimeDelta {
        self.end - self.start
    }

    pub fn previous(&self) -> Self {
        let prev_start = self.start - self.duration();

        Self {
            start: prev_start,
            end: self.start,
        }
    }

    pub fn in_timespan(&self, date: DateTime<Utc>) -> bool {
        self.start <= date && date <= self.end
    }
}

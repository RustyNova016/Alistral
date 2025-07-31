use chrono::DateTime;
use chrono::Utc;

use crate::datastructures::entity_with_listens::listen_timeframe::extract_timeframe::ExtractTimeframe;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;

pub mod extract_timeframe;

#[derive(Debug, Clone)]
pub struct ListenTimeframe<T>
where
    T: ExtractTimeframe + ListenCollectionReadable,
{
    /// The start of the previous timeframe (inclusive)
    prev_start: DateTime<Utc>,
    /// The start of the timeframe (inclusive)
    start: DateTime<Utc>,
    /// The end of the timeframe (inclusive)
    end: DateTime<Utc>,

    all_time: T,
    current: T,
    previous: T,
}

impl<T> ListenTimeframe<T>
where
    T: ExtractTimeframe + ListenCollectionReadable,
{
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, all_time: T) -> Self
    where
        T: Clone,
    {
        // Let's find the previous start date
        let dur = end - start;
        let prev_start = start - dur;

        // Get the data
        let current = all_time.clone().extract_timeframe(start, end, true, true);
        let previous = all_time
            .clone()
            .extract_timeframe(prev_start, start, true, true);

        Self {
            current,
            previous,
            all_time,
            prev_start,
            end,
            start,
        }
    }

    pub fn all_time(&self) -> &T {
        &self.all_time
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn previous(&self) -> &T {
        &self.previous
    }

    /// Return the previous timeframe, and return none if no listens are in it
    pub fn previous_opt(&self) -> Option<&T> {
        if self.previous.listen_count() == 0 {
            return None;
        }
        Some(&self.previous)
    }

    pub fn prev_start(&self) -> DateTime<Utc> {
        self.prev_start
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }
}

pub enum ListenTimeframeOrFull<T>
where
    T: ExtractTimeframe + ListenCollectionReadable,
{
    Full(T),
    Timeframe(ListenTimeframe<T>),
}

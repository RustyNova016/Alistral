use crate::datastructures::entity_with_listens::listen_timeframe::extract_timeframe::ExtractTimeframe;

pub mod extract_timeframe;
pub mod timewindow;


pub struct ListenTimeframe<T: ExtractTimeframe> {
    /// The start of the timeframe (inclusive)
    start: DateTime<Utc>,
    /// The end of the timeframe (inclusive)
    end: DateTime<Utc>,


    all_time: T,
    current: T,
    previous: T,
}

impl<T: ExtractTimeframe> ListenTimeframe<T> {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, all_time: T) -> Self
    where
        T: Clone,
    {
        Self {
            current: timeframe.get_current_data(all_time.clone()),
            previous: timeframe.get_previous_data(all_time.clone()),
            all_time,
            settings: timeframe,
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
}

pub enum ListenTimeframeOrFull<T> {
    Full(T),
    Timeframe(ListenTimeframe)
}
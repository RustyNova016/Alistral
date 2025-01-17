use chrono::Duration;

pub trait ListenCollWithTime {
    /// Return the total listens time all the listens in the collection
    fn get_time_listened(&self) -> Option<Duration>;
}

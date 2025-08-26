use chrono::DateTime;
use chrono::Utc;

/// A trait for extracting a timeframe of data from a data structure.
pub trait ExtractTimeframe {
    /// A trait for extracting a timeframe of data from a data structure.
    ///
    /// # Parameters
    ///
    /// - `start`: The start of the timeframe as a `DateTime<Utc>`.
    /// - `end`: The end of the timeframe as a `DateTime<Utc>`.
    /// - `include_start`: A boolean indicating whether to include the start time in the extracted timeframe.
    /// - `include_end`: A boolean indicating whether to include the end time in the extracted timeframe.
    fn extract_timeframe(
        self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_start: bool,
        include_end: bool,
    ) -> Self;
}

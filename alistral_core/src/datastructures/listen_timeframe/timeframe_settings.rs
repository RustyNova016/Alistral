use crate::datastructures::listen_timeframe::time_windows::TimeWindow;
use crate::datastructures::listen_timeframe::traits::ExtractTimeframe;

pub struct TimeframeSettings {
    timeframe: TimeWindow,

    include_start: bool,
    include_end: bool,
}

impl TimeframeSettings {
    pub fn new(timeframe: TimeWindow, include_start: bool, include_end: bool) -> Self {
        Self {
            timeframe,
            include_start,
            include_end,
        }
    }

    pub fn get_current_data<T: ExtractTimeframe>(&self, all_time_data: T) -> T {
        all_time_data.extract_timeframe(
            self.timeframe.start(),
            self.timeframe.end(),
            self.include_start,
            self.include_end,
        )
    }

    pub fn get_previous_data<T: ExtractTimeframe>(&self, all_time_data: T) -> T {
        all_time_data.extract_timeframe(
            self.timeframe.previous_start(),
            self.timeframe.start(),
            self.include_start,
            !self.include_start,
        )
    }

    pub fn timeframe(&self) -> &TimeWindow {
        &self.timeframe
    }
}

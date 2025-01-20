use crate::datastructures::listen_timeframe::timeframe_settings::TimeframeSettings;
use crate::datastructures::listen_timeframe::traits::ExtractTimeframe;

pub mod time_windows;
pub mod traits;
pub mod timeframe_settings;

pub struct ListenTimeframe<T: ExtractTimeframe> {
    settings: TimeframeSettings,

    all_time: T,
    current: T,
    previous: T,
}

impl<T: ExtractTimeframe> ListenTimeframe<T> {
    pub fn new(timeframe: TimeframeSettings, all_time: T) -> Self where T: Clone {
        Self {
            current: timeframe.get_current_data(all_time.clone()),
            previous: timeframe.get_previous_data(all_time.clone()),
            all_time,
            settings: timeframe
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
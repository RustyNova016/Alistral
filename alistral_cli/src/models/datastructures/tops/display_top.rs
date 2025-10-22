use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

/// Trait for all the data displayed in the top
pub trait DisplayTop {
    fn to_string_top(&self) -> String;
}

impl DisplayTop for usize {
    fn to_string_top(&self) -> String {
        self.to_string()
    }
}

impl DisplayTop for Duration {
    fn to_string_top(&self) -> String {
        self.floor_to_minute().to_humantime().unwrap().to_string()
    }
}
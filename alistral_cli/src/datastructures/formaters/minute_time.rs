use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

pub struct MinuteTimePrinter(Option<Duration>);

impl Display for MinuteTimePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dur) = self.0 {
            write!(f, "{}m", dur.floor_to_minute().num_minutes())?
        }

        Ok(())
    }
}

impl From<Option<Duration>> for MinuteTimePrinter {
    fn from(value: Option<Duration>) -> Self {
        Self(value)
    }
}

impl From<Duration> for MinuteTimePrinter {
    fn from(value: Duration) -> Self {
        Self(Some(value))
    }
}

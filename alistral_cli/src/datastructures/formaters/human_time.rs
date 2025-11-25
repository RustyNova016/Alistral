use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

pub struct HumanTimePrinter(Option<Duration>);

impl Display for HumanTimePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(dur) => write!(
                f,
                "{}",
                dur.floor_to_minute().to_humantime().unwrap().to_string()
            )
            .unwrap(),
            None => {}
        }

        Ok(())
    }
}

impl From<Option<Duration>> for HumanTimePrinter {
    fn from(value: Option<Duration>) -> Self {
        Self(value)
    }
}

impl From<Duration> for HumanTimePrinter {
    fn from(value: Duration) -> Self {
        Self(Some(value))
    }
}

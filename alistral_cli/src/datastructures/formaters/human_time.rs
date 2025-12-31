use core::cmp::Ordering;
use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HumanTimePrinter(Option<Duration>);

impl Display for HumanTimePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dur) = self.0 {
            write!(f, "{}", dur.floor_to_minute().to_humantime().unwrap()).unwrap()
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

impl PartialOrd for HumanTimePrinter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HumanTimePrinter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.unwrap_or_default().cmp(&other.0.unwrap_or_default())
    }
}

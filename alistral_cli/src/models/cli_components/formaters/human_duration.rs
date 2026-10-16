use core::fmt::Display;

use chrono::TimeDelta;
use tuillez::extensions::chrono_exts::DurationExt;

pub struct HumanDurationFormat(pub TimeDelta);

impl From<TimeDelta> for HumanDurationFormat {
    fn from(value: TimeDelta) -> Self {
        Self(value)
    }
}

impl From<Option<TimeDelta>> for HumanDurationFormat {
    fn from(value: Option<TimeDelta>) -> Self {
        Self(value.unwrap_or_default())
    }
}

impl Display for HumanDurationFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = self.0.num_seconds();
        let truncated = TimeDelta::new(seconds, 0).unwrap();

        write!(f, "{}", truncated.to_humantime().unwrap_or_default())
    }
}

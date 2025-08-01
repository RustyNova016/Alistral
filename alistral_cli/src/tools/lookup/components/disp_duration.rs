use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

pub struct DispDuration {
    pub data: Duration,
}

impl Display for DispDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.to_humantime().unwrap())
    }
}

use core::fmt::Display;

use tuillez::extensions::chrono_exts::DurationExt as _;

/// Score for a top
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum TopScore {
    TimeDelta(chrono::TimeDelta),
}

impl Display for TopScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeDelta(dur) => write!(f, "{}", dur.floor_to_minute().to_humantime().unwrap()),
        }
    }
}

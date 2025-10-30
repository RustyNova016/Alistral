use core::fmt::Display;

use tuillez::extensions::chrono_exts::DurationExt as _;

/// Score for a top
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum TopScore {
    Number(i64),
    TimeDelta(chrono::TimeDelta),
}

impl Display for TopScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::TimeDelta(dur) => write!(f, "{}", dur.floor_to_minute().to_humantime().unwrap().to_string()),
        }
    }
}

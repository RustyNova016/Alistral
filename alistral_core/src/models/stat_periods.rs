use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// A bunch of preset time ranges that an user might want to use
#[derive(Clone, Debug, Copy, Default, Serialize, Deserialize)]
pub enum StatPeriod {
    /// Uses the last 30 days from now
    Last30Days,

    /// Uses the last 30 days from now   
    #[default]
    Last90Days,

    /// Uses the last 365 days from now    
    Last365Days,

    AllTime,
}

impl StatPeriod {
    pub fn get_start_date(&self) -> DateTime<Utc> {
        let now = Utc::now();

        match self {
            Self::Last30Days => now - Duration::days(30),
            Self::Last90Days => now - Duration::days(90),
            Self::Last365Days => now - Duration::days(365),
            Self::AllTime => DateTime::<Utc>::MIN_UTC,
        }
    }

    pub fn get_end_date(&self) -> DateTime<Utc> {
        let now = Utc::now();

        match self {
            Self::Last30Days => now,
            Self::Last90Days => now,
            Self::Last365Days => now,
            Self::AllTime => DateTime::<Utc>::MAX_UTC,
        }
    }
}

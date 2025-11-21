use core::fmt::Display;

#[cfg(any(feature = "radio", feature = "lookup", feature = "stats"))]
use chrono::DateTime;
#[cfg(any(feature = "radio", feature = "lookup", feature = "stats"))]
use chrono::Duration;
#[cfg(any(feature = "radio", feature = "lookup", feature = "stats"))]
use chrono::Utc;
use clap::ValueEnum;
use derive_more::IsVariant;

#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum SortListensBy {
    #[default]
    None,
    Name,
    OldestListen,
}

#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum SortSorterBy {
    /// The count of listens for this element. This is descending by default
    #[default]
    Count,

    /// The name of the associated element
    Name,

    /// The oldest element
    Oldest,
    // /// The oldest listen
}

impl Display for SortSorterBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Name => write!(f, "name"),
            Self::Oldest => write!(f, "oldest"),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum ConfigBool {
    #[default]
    Toggle,
    True,
    False,
}

#[expect(
    clippy::enum_variant_names,
    reason = "This is used for the CLI. So the postfix must stay"
)]
#[derive(ValueEnum, Clone, Debug, Copy, Default, IsVariant)]
pub enum Timeframe {
    /// Uses the last 30 days from now
    #[default]
    Last30Days,

    /// Uses the last 30 days from now    
    Last90Days,

    /// Uses the last 365 days from now    
    Last365Days,
}

impl Timeframe {
    //#[cfg(any(feature = "radio", feature = "lookup", feature = "stats"))]
    #[cfg(any(feature = "radio", feature = "lookup"))]
    pub fn get_start_date(&self) -> DateTime<Utc> {
        let now = Utc::now();

        match self {
            Self::Last30Days => now - Duration::days(30),
            Self::Last90Days => now - Duration::days(90),
            Self::Last365Days => now - Duration::days(365),
        }
    }
}

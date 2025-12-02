use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum SortBy {
    /// The number of times the entity has been listened to
    ListenCount,

    /// The total duration this entity has been listened for
    ListenDuration,
}

impl SortBy {}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListenCount => write!(f, "listen-count")?,
            Self::ListenDuration => write!(f, "listen-duration")?,
        };

        Ok(())
    }
}

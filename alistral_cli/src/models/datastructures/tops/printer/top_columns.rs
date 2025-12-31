use core::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TopColumnType {
    /// Show the ranking of the entity
    Rank,

    /// Show the name of the entity
    Title,

    /// Show the listen counts of the entity
    ListenCount,

    /// Show the listen duration of the entity
    ListenDuration,
}

impl Display for TopColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListenCount => write!(f, "Listen Count"),
            Self::ListenDuration => write!(f, "Listen Duration"),
            Self::Rank => write!(f, "Rank"),
            Self::Title => write!(f, "Name"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TopColumnSort {
    #[expect(dead_code)]
    Asc,
    Desc,
}

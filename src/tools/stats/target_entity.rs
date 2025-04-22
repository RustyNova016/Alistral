use clap::ValueEnum;
use derive_more::IsVariant;
#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum StatsTarget {
    Recording,
    Artist,
    Release,
    ReleaseGroup,
    Work,
    WorkRecursive,
    Tag,
}

impl StatsTarget {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Recording => "recording",
            Self::Artist => "artist",
            Self::Release => "release",
            Self::ReleaseGroup => "release_group",
            Self::Work => "work",
            Self::WorkRecursive => "work_recursive",
            Self::Tag => "tag",
        }
    }
}

use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use clap::ValueEnum;
use derive_more::IsVariant;
use musicbrainz_db_lite::RowId;

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum StatsTarget {
    Recording,
    Artist,
    Release,
    ReleaseGroup,
    Work,
    WorkRecursive,
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
        }
    }
}

/// Compiled stat data
pub struct StatData<Ent: RowId>(EntityWithListensCollection<Ent, ListenCollection>);
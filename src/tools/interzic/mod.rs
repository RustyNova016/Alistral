use clap::ValueEnum;
use strum::Display;
use strum::EnumIter;

pub mod get_mapping;
pub mod overwrite;
pub mod reload;
pub mod reverse_mapping;

#[derive(ValueEnum, Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum IdOrigin {
    Youtube,
    Musicbrainz,
}

#[derive(ValueEnum, Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum InterzicMappingTarget {
    Youtube,
}

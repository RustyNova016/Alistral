pub mod api;
pub mod database;
pub mod error;
pub mod models;
pub mod utils;

pub use musicbrainz_rs::*;

pub use crate::database::client::DBClient;
pub use crate::error::Error;

// === Entity ===
pub use crate::models::musicbrainz::artist::Artist;
pub use crate::models::musicbrainz::artist_credit::ArtistCredit;
pub use crate::models::musicbrainz::artist_credit::ArtistCredits;
pub use crate::models::musicbrainz::genre::Genre;
pub use crate::models::musicbrainz::label::Label;
pub use crate::models::musicbrainz::main_entities::MainEntity;
pub use crate::models::musicbrainz::recording::Recording;
pub use crate::models::musicbrainz::relations::Relation;
pub use crate::models::musicbrainz::release::LabelInfo;
pub use crate::models::musicbrainz::release::Media;
pub use crate::models::musicbrainz::release::Release;
pub use crate::models::musicbrainz::release_group::ReleaseGroup;
pub use crate::models::musicbrainz::tags::Tag;
pub use crate::models::musicbrainz::track::Track;
pub use crate::models::musicbrainz::url::Url;
pub use crate::models::musicbrainz::user::User;
pub use crate::models::musicbrainz::work::Work;

// === Traits ===
pub use crate::models::shared_traits::HasMBID;
pub use crate::models::shared_traits::RowId;
pub use crate::models::shared_traits::completeness::CompletenessFlag;
pub use crate::models::shared_traits::completeness::FetchAsComplete;
pub use crate::models::shared_traits::db_relation::DBRelation;
pub use crate::models::shared_traits::db_relation::complete_fetch::DBRelationFetch;
pub use crate::models::shared_traits::fetch_and_save::FetchAndSave;
pub use crate::models::shared_traits::find_by::FindBy;
pub use crate::models::shared_traits::find_by_mbid::FindByMBID;
pub use crate::models::shared_traits::find_by_rowid::FindByRowID;
pub use crate::models::shared_traits::get_or_fetch::GetOrFetch;
pub use crate::models::shared_traits::has_urls::HasUrls;
pub use crate::models::shared_traits::mbid_redirection::MBIDRedirection;
pub use crate::models::shared_traits::save_from::SaveFrom;

// === Internal Reexports ===
pub(crate) type MBLabel = musicbrainz_rs::entity::label::Label;
pub(crate) type MBRecording = musicbrainz_rs::entity::recording::Recording;
pub(crate) type MBRelease = musicbrainz_rs::entity::release::Release;
pub(crate) type MBReleaseGroup = musicbrainz_rs::entity::release_group::ReleaseGroup;
pub(crate) type MBUrl = musicbrainz_rs::entity::url::Url;

pub mod sequelle {
    pub use sequelles::*;
}

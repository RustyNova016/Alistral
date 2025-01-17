use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub mod collection;

pub type ReleaseGroupWithListens = EntityWithListens<ReleaseGroup, ListenCollection>;

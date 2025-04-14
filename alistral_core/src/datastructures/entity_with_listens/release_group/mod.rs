use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;

use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::datastructures::entity_with_listens::EntityWithListens;

pub mod collection;

pub type ReleaseGroupWithReleases =
    EntityWithListens<ReleaseGroup, ReleaseWithRecordingsCollection>;

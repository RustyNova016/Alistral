use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::EntityWithListens;

pub mod collection;

pub type ReleaseWithRecordings = EntityWithListens<Release, RecordingWithListensCollection>;

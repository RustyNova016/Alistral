use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub mod collection;

pub type ReleaseWithListens = EntityWithListens<Release, ListenCollection>;

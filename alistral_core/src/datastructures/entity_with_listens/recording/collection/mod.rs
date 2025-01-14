use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;

pub type RecordingWithListensCollection = EntityWithListensCollection<Recording, ListenCollection>;

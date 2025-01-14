pub mod collection;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::datastructures::listen_collection::ListenCollection;

use super::EntityWithListens;

pub type RecordingWithListens = EntityWithListens<Recording, ListenCollection>;

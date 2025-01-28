use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::EntityWithListens;

pub type WorkWithRecordings = EntityWithListens<Work, RecordingWithListensCollection>;

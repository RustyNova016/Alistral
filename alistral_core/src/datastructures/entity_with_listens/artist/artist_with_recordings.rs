use musicbrainz_db_lite::models::musicbrainz::artist::Artist;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;

use super::EntityWithListens;

pub type ArtistWithRecordings = EntityWithListens<Artist, RecordingWithListensCollection>;


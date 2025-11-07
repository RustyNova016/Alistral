use chrono::TimeDelta;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::traits::ListenCollWithTime;

use super::EntityWithListens;

pub type ArtistWithRecordings = EntityWithListens<Artist, RecordingWithListensCollection>;


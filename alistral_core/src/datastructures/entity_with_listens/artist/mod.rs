pub mod artist_with_recordings;
pub mod collection;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;

use crate::datastructures::listen_collection::ListenCollection;

use super::EntityWithListens;

pub type ArtistWithListens = EntityWithListens<Artist, ListenCollection>;

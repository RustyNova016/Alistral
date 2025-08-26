pub mod collection;
use musicbrainz_db_lite::User;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub type UserWithListens = EntityWithListens<User, ListenCollection>;

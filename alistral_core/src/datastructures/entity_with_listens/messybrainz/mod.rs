use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub mod collection;

pub type MessybrainzWithListens = EntityWithListens<MessybrainzSubmission, ListenCollection>;

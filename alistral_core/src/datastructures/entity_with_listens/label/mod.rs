use musicbrainz_db_lite::Label;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;

pub mod collection;

pub type LabelWithReleases = EntityWithListens<Label, ReleaseWithRecordingsCollection>;

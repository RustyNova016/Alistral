//! Implementation for statistic queries

use chrono::Duration;
use musicbrainz_db_lite::HasRowID;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::traits::IterRecordingWithListens;
use crate::datastructures::entity_with_listens::traits::ListenCollWithTime;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;

impl<Ent, Lis> ListenCollWithTime for EntityWithListens<Ent, Lis>
where
    Self: IterRecordingWithListens,
    Ent: HasRowID,
    Lis: ListenCollectionReadable,
{
    fn get_time_listened(&self) -> Option<Duration> {
        Some(
            self.iter_recording_with_listens()
                .map(|rec| rec.get_time_listened().unwrap_or_else(Duration::zero))
                .sum(),
        )
    }
}

use core::fmt::Debug;

use chrono::DateTime;
use chrono::Utc;
use sequelles::bon::__::alloc::sync::Arc;

use crate::DBClient;

pub trait DebutedOn {
    type Error: Debug;

    /// Date of the debut of an entity. This means different things for different entities:
    /// - Recordings / Release Groups / Work: it's the time they first released
    /// - Release: Release date
    /// - Artist / Label: the date of their first associated release
    fn debuted_on(&self, client: &Arc<DBClient>) -> impl Future<Output = Result<Option<DateTime<Utc>>, Self::Error>> + Send;
}

use crate::datastructures::entity_with_listens::recording_with_listens::collection::RecordingWithListensCollection;

use super::lookup_trait::LookupTrait;
use super::Lookup;

pub struct RecordingLookup;

impl Lookup<RecordingWithListensCollection, RecordingLookup> {
    pub async fn empty_lookup(&self) -> Result<String, crate::Error> {
        Ok(format!(
            "{}
                    
        The recording hasn't been listened to yet",
            self.get_title(conn).await?,
        ))
    }
}

impl LookupTrait for Lookup<RecordingWithListensCollection, RecordingLookup> {
    async fn to_string(&self) -> Result<String, crate::Error> {}
}

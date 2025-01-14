use core::cmp::Reverse;
use std::collections::HashMap;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::datastructures::listen_collection::ListenCollection;
use futures::stream;
use futures::Stream;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use rust_decimal::Decimal;

use crate::database::listenbrainz::prefetching::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::entity_with_listen_collection::EntityWithListensCollection;

use super::RecordingWithListensOld;

/// An `HashMap` containing `RecordingWithListens`, indexed on the Recording's ID
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[deprecated]
pub struct RecordingWithListensCollectionOld(pub HashMap<i64, RecordingWithListensOld>);

impl RecordingWithListensCollectionOld {
    pub fn iter_recordings(&self) -> impl Iterator<Item = &Recording> {
        self.0.values().map(|r| r.recording())
    }

    pub fn values(&self) -> impl Iterator<Item = &RecordingWithListensOld> {
        self.0.values()
    }

    pub fn into_iter_recordings(self) -> impl Iterator<Item = Recording> {
        self.0.into_values().map(|r| r.recording)
    }

    pub fn into_values(self) -> impl Iterator<Item = RecordingWithListensOld> {
        self.0.into_values()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_by_id(&self, id: i64) -> Option<&RecordingWithListensOld> {
        self.0.get(&id)
    }

    pub fn get_by_mbid(&self, mbid: &str) -> Option<&RecordingWithListensOld> {
        self.0.values().find(|r| r.recording().mbid == mbid)
    }

    pub fn get_or_new(&mut self, recording: Recording) -> &RecordingWithListensOld {
        self.0
            .entry(recording.id)
            .or_insert_with(|| RecordingWithListensOld::new(recording, Default::default()))
    }

    /// Return the ratio of listens being from a recording
    pub fn get_listen_ratio(&self, recording: &Recording) -> Decimal {
        let recording_listen_count = self
            .get_by_id(recording.id)
            .map(|r| r.listen_count())
            .unwrap_or(0);

        Decimal::new(recording_listen_count.try_into().unwrap(), 0)
            / Decimal::new(self.listen_count().try_into().unwrap(), 0)
    }

    pub fn into_values_stream(self) -> impl Stream<Item = RecordingWithListensOld> {
        stream::iter(self.0.into_values())
    }

    /// Remove all the recordings with less or equal than the provided count
    pub fn retain_more_than_count(&mut self, count: usize) {
        let mut deathlist = Vec::new();

        for (id, entity) in &self.0 {
            if entity.listen_count() <= count {
                deathlist.push(*id);
            }
        }

        for id in deathlist {
            self.0.remove(&id);
        }
    }

    pub fn merge(&mut self, other: Self) {
        for (new_id, new_entity) in other.0 {
            if let Some(old_entity) = self.0.get_mut(&new_id) {
                old_entity.merge(new_entity);
            } else {
                self.0.insert(new_id, new_entity);
            }
        }
    }

    /// Return the rank of the listened recording
    pub fn get_rank(&self, recording_mbid: &str) -> Option<usize> {
        let mut rec_ref = self.values().collect_vec();
        rec_ref.sort_by_cached_key(|rec| Reverse(rec.listen_count()));

        let mut rank = 1;
        let mut last_listen_count = 0;
        for (i, rec) in rec_ref.iter().enumerate() {
            if last_listen_count != rec.listen_count() {
                rank = i;
            }

            if rec.recording().mbid == recording_mbid {
                return Some(rank);
            }

            last_listen_count = rec.listen_count();
        }

        None
    }
}

impl ListenCollectionReadable for RecordingWithListensCollectionOld {
    fn iter_listens(
        &self,
    ) -> impl Iterator<Item = &musicbrainz_db_lite::models::listenbrainz::listen::Listen> {
        self.0.values().flat_map(|l| l.iter_listens())
    }
}

impl From<HashMap<i64, RecordingWithListensOld>> for RecordingWithListensCollectionOld {
    fn from(value: HashMap<i64, RecordingWithListensOld>) -> Self {
        Self(value)
    }
}

impl From<RecordingWithListensCollectionOld> for HashMap<i64, RecordingWithListensOld> {
    fn from(value: RecordingWithListensCollectionOld) -> Self {
        value.0
    }
}

impl From<Vec<RecordingWithListensOld>> for RecordingWithListensCollectionOld {
    fn from(value: Vec<RecordingWithListensOld>) -> Self {
        let mut out = HashMap::new();

        for val in value {
            out.insert(val.recording().id, val);
        }

        out.into()
    }
}

pub type RecordingWithListensCollection = EntityWithListensCollection<Recording, ListenCollection>;

impl RecordingWithListensCollection {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(Default::default());
        }

        // Prefetch the missing data
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, user.id, &listens.data).await?;

        // Get all the data from the DB
        let joins = Listen::get_recordings_as_batch(conn, user.id, listens.data).await?;

        // Convert into structs
        let mut out = Self::new();

        for (_, (listen, recordings)) in joins {
            for recording in recordings {
                out.insert_or_merge_listen(recording, listen.clone());
            }
        }

        Ok(out.into())
    }
}
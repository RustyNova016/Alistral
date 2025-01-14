use core::cmp::Reverse;
use core::ops::Deref;

use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use serde::Deserialize;
use serde::Serialize;

pub mod traits;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct ListenCollection {
    pub data: Vec<Listen>,
}

impl ListenCollection {
    pub fn new(data: Vec<Listen>) -> Self {
        Self { data }
    }

    /// Return the latest X listens in the collection.
    pub fn get_latest_listens(&self, limit: usize) -> Self {
        let mut slice = self.data.clone();
        slice.sort_by_key(|listen| Reverse(listen.listened_at));

        Self::from(
            slice
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i >= &limit)
                .map(|(_, l)| l)
                .collect_vec(),
        )
    }

    pub fn push(&mut self, listen: Listen) {
        self.data.push(listen);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the first element of the collection
    ///
    /// To return the oldest listen use `ListenCollection::get_oldest_listen`
    pub fn first(&self) -> Option<&Listen> {
        self.data.first()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Listen> {
        self.data.iter()
    }

    /// Merge two collections into one, removing duplicate listens by the index (`listened_at`, `recording_msid`, `user`)
    pub fn merge_by_index(&mut self, other: Self) {
        for new_listen in other.data {
            self.push_unique(new_listen);
        }
    }

    /// Push the listen if it isn't already in the collection
    pub fn push_unique(&mut self, new_listen: Listen) {
        if !self.data.iter().any(|listen| {
            listen.listened_at == new_listen.listened_at
                && listen.recording_msid == new_listen.recording_msid
                && listen.user == new_listen.user
        }) {
            self.data.push(new_listen);
        }
    }
}

impl From<Listen> for ListenCollection {
    fn from(value: Listen) -> Self {
        Self { data: vec![value] }
    }
}

impl From<Vec<Listen>> for ListenCollection {
    fn from(value: Vec<Listen>) -> Self {
        Self { data: value }
    }
}

impl Deref for ListenCollection {
    type Target = Vec<Listen>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use crate::RowIDMap;
use crate::has_rowid::HasRowID;

/// An [`crate::RowIDMap`] that represent a `LEFT JOIN`, where an element of the Left table <u>can</u> have <u>one</u> element of the Right table
///
/// Example: a **Recording can have <u>many</u> Listens**, but a Listen <u>can</u> have a Recording
pub struct ZeroToManyJoin<L, R>(pub(super) RowIDMap<Option<L>, Vec<R>>);

impl<L, R> ZeroToManyJoin<L, R>
where
    L: HasRowID,
{
    pub fn insert(&mut self, key: Option<L>, value: Vec<R>) {
        self.0.insert(key, value);
    }

    /// Push a value to its correponding entry
    pub fn push_entry(&mut self, key: Option<L>, value: R) {
        self.0
            .as_mut_hash_map()
            .entry(key.rowid())
            .or_insert((key, Vec::new()))
            .1
            .push(value);
    }

    /// Push multiple values to its correponding entry
    pub fn push_entries(&mut self, key: Option<L>, value: Vec<R>) {
        self.0
            .as_mut_hash_map()
            .entry(key.rowid())
            .or_insert((key, Vec::new()))
            .1
            .extend(value);
    }

    pub fn insert_left(&mut self, left: Option<L>) {
        self.0.insert(left, Vec::new());
    }

    /// Add a right value using an id. If the left value doesn't exists, it won't be inserted
    pub fn push_right_by_id(&mut self, key: i64, value: R) {
        self.as_mut_hash_map()
            .entry(key)
            .and_modify(|vals| vals.1.push(value));
    }

    pub fn as_mut_hash_map(&mut self) -> &mut HashMap<i64, (Option<L>, Vec<R>)> {
        self.0.as_mut_hash_map()
    }

    pub fn map_left<F, U>(self, f: F) -> ZeroToManyJoin<U, R>
    where
        F: Fn(L) -> U,
        U: HasRowID,
    {
        let mut new_map = ZeroToManyJoin::default();

        for (left, right) in self {
            let left = left.map(&f);
            new_map.insert(left, right);
        }

        new_map
    }

    pub fn map_right<F, U>(self, f: F) -> ZeroToManyJoin<L, U>
    where
        F: FnMut(R) -> U + Clone,
    {
        let mut new_map = ZeroToManyJoin::default();

        for (left, right) in self {
            let right = right.into_iter().map(f.clone()).collect::<Vec<_>>();
            new_map.insert(left, right);
        }

        new_map
    }
}

impl<L, R> Default for ZeroToManyJoin<L, R> {
    fn default() -> Self {
        Self(RowIDMap::default())
    }
}

impl<L, R> IntoIterator for ZeroToManyJoin<L, R> {
    type Item = (Option<L>, Vec<R>);
    type IntoIter = IntoValues<i64, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

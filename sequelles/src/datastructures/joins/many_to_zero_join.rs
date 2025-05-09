use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use crate::RowIDMap;
use crate::datastructures::joins::zero_to_many_join::ZeroToManyJoin;
use crate::has_rowid::HasRowID;

/// An [`crate::RowIDMap`] that represent a `LEFT JOIN`, where an element of the Left table <u>can</u> have <u>one</u> element of the Right table
///
/// Example: **a Listen <u>can</u> have a Recording**, but a Recording can have <u>many</u> Listens
pub struct ManyToZeroJoin<L, R>(pub(super) RowIDMap<L, Option<R>>);

impl<L, R> ManyToZeroJoin<L, R>
where
    L: HasRowID,
{
    /// Insert a key-value pair
    pub fn insert(&mut self, left: L, right: Option<R>) {
        self.0.insert(left, right);
    }

    /// Replace the value at a specific rowid
    pub fn replace_by_id(&mut self, key: i64, value: R) -> Option<R> {
        self.0.get_mut_by_id(key).and_then(|val| val.replace(value))
    }

    pub fn invert(self) -> ZeroToManyJoin<R, L>
    where
        R: HasRowID,
    {
        let mut new_map = ZeroToManyJoin::default();

        for (left, right) in self.0.into_iter() {
            new_map.push_entry(right, left);
        }

        new_map
    }

    /// Return the underlying hashmap.
    pub fn as_mut_hashmap(&mut self) -> &mut HashMap<i64, (L, Option<R>)> {
        self.0.as_mut_hash_map()
    }

    pub fn map_left<F, U>(self, f: F) -> ManyToZeroJoin<U, R>
    where
        F: Fn(L) -> U,
        U: HasRowID,
    {
        let mut new_map = ManyToZeroJoin::default();

        for (left, right) in self {
            let left = f(left);
            new_map.insert(left, right);
        }

        new_map
    }

    pub fn map_right<F, U>(self, f: F) -> ManyToZeroJoin<L, U>
    where
        F: Fn(R) -> U,
    {
        let mut new_map = ManyToZeroJoin::default();

        for (left, right) in self {
            let right = right.map(&f);
            new_map.insert(left, right);
        }

        new_map
    }
}

impl<L, R> Default for ManyToZeroJoin<L, R> {
    fn default() -> Self {
        Self(RowIDMap::default())
    }
}

impl<L, R> IntoIterator for ManyToZeroJoin<L, R> {
    type Item = (L, Option<R>);
    type IntoIter = IntoValues<i64, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

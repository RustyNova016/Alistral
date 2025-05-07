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
    /// Push a value to its correponding entry
    pub fn push_entry(&mut self, key: Option<L>, value: R) {
        self.0
            .as_mut_hash_map()
            .entry(key.rowid())
            .or_insert((key, Vec::new()))
            .1
            .push(value);
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

use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use crate::has_rowid::HasRowID;

/// A database table. This allow easy access to any item using its rowid
pub struct Table<R>(HashMap<i64, R>);

impl<R> Table<R>
where
    R: HasRowID,
{
    /// Create a new table
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Insert a new row in the table
    pub fn insert(&mut self, value: R) {
        self.0.insert(value.rowid(), value);
    }

    /// Get a row by its rowid
    pub fn get(&self, key: &i64) -> Option<&R> {
        self.0.get(key)
    }

    /// Get a mutable reference to a row by its rowid
    pub fn get_mut(&mut self, key: &i64) -> Option<&mut R> {
        self.0.get_mut(key)
    }

    /// Remove a value from the table
    pub fn remove(&mut self, key: &i64) -> Option<R> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, i64, R> {
        self.0.values()
    }
}

impl<R> Default for Table<R> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<R> From<Vec<R>> for Table<R>
where
    R: HasRowID,
{
    fn from(value: Vec<R>) -> Self {
        let mut table = Self::new();
        for item in value {
            table.insert(item);
        }
        table
    }
}

impl<R> IntoIterator for Table<R> {
    type Item = R;
    type IntoIter = IntoValues<i64, R>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

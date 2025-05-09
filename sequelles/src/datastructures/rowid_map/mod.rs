use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use crate::tables::traits::has_rowid::HasRowID;

/// An hashmap that use the rowid of its "key" element as actual key, relieving it from the Eq + Hash requirement
pub struct RowIDMap<K, V>(HashMap<i64, (K, V)>);

impl<K, V> RowIDMap<K, V>
where
    K: HasRowID,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key.rowid(), (key, value));
    }

    pub fn get_by_id(&self, key: i64) -> Option<&V> {
        self.0.get(&key).map(|(_, val)| val)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_by_id(key.rowid())
    }

    pub fn get_mut_by_id(&mut self, key: i64) -> Option<&mut V> {
        self.0.get_mut(&key).map(|(_, val)| val)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut_by_id(key.rowid())
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(&key.rowid())
    }

    pub fn contains_id(&self, key: i64) -> bool {
        self.0.contains_key(&key)
    }

    pub fn as_hash_map(&self) -> &HashMap<i64, (K, V)> {
        &self.0
    }

    pub fn as_mut_hash_map(&mut self) -> &mut HashMap<i64, (K, V)> {
        &mut self.0
    }
}

impl<K, V> Default for RowIDMap<K, V> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<K, V> IntoIterator for RowIDMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoValues<i64, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

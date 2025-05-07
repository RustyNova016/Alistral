use std::collections::HashMap;

use crate::RowId;

pub struct Index<T>(pub HashMap<i64, T>);

impl<T> Index<T>
where
    T: RowId,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, value: T) {
        self.0.insert(value.get_row_id(), value);
    }

    pub fn get(&self, id: i64) -> Option<&T> {
        self.0.get(&id)
    }

    pub fn remove(&mut self, id: i64) -> Option<T> {
        self.0.remove(&id)
    }

    pub fn contains_key(&self, id: i64) -> bool {
        self.0.contains_key(&id)
    }
}

impl<T> From<Vec<T>> for Index<T>
where
    T: RowId,
{
    fn from(value: Vec<T>) -> Self {
        let mut index = Index::new();

        for item in value {
            index.insert(item);
        }

        index
    }
}

pub trait IntoIndexes<L, R> {
    fn into_indexes(self) -> (Index<L>, Index<R>);
}

pub trait ApplyIndexes<L, R> {
    fn apply_indexes(self, left_index: Index<L>, right_index: Index<R>);
}

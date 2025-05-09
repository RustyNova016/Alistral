use std::collections::HashMap;

use crate::Table;
use crate::ZeroToManyJoin;
use crate::has_rowid::HasRowID;

/// Represent a Many to Many join in the database.
///
/// While it can be useful, prefer using [`ManyToZeroJoin`](crate::ManyToZeroJoin) or [`ZeroToManyJoin`] when possible, as they take less memory and promote less cloning
pub struct ManyToManyJoin<L, R> {
    left_table: Table<L>,
    right_table: Table<R>,

    left_to_right: HashMap<i64, Vec<i64>>,
    right_to_left: HashMap<i64, Vec<i64>>,
}

impl<L, R> ManyToManyJoin<L, R>
where
    L: HasRowID,
    R: HasRowID,
{
    /// Add a new element to the left table
    pub fn add_left(&mut self, left: L) {
        self.left_table.insert(left);
    }

    /// Add a new element to the right table
    pub fn add_right(&mut self, right: R) {
        self.right_table.insert(right);
    }

    /// Add a new relation between a left element and a right element using their rowids
    pub fn add_relation_ids(&mut self, left: i64, right: i64) {
        self.left_to_right.entry(left).or_default().push(right);
        self.right_to_left.entry(right).or_default().push(left);
    }

    /// Add a new relation between a left element and a right element
    pub fn add_relation(&mut self, left: &L, right: &R) {
        self.add_relation_ids(left.rowid(), right.rowid());
    }

    /// Remove a relation between a left element and a right element using their rowids
    pub fn remove_relation_ids(&mut self, left: i64, right: i64) {
        if let Some(left_vec) = self.left_to_right.get_mut(&left) {
            left_vec.retain(|&x| x != right);
        }
        if let Some(right_vec) = self.right_to_left.get_mut(&right) {
            right_vec.retain(|&x| x != left);
        }
    }

    /// Remove a relation between a left element and a right element
    pub fn remove_relation(&mut self, left: &L, right: &R) {
        self.remove_relation_ids(left.rowid(), right.rowid());
    }

    /// Get a left element by its rowid
    pub fn get_left(&self, key: &i64) -> Option<&L> {
        self.left_table.get(key)
    }

    /// Get a right element by its rowid
    pub fn get_right(&self, key: &i64) -> Option<&R> {
        self.right_table.get(key)
    }

    /// Get all associated right elements to a left element by its rowid
    pub fn get_associated_rights_by_id(&self, left: i64) -> Vec<&R> {
        self.left_to_right
            .get(&left)
            .map(|r_ids| {
                r_ids
                    .iter()
                    .filter_map(|id| self.right_table.get(id))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Get all associated right elements to a left element
    pub fn get_associated_rights(&self, left: &L) -> Vec<&R> {
        self.get_associated_rights_by_id(left.rowid())
    }

    /// Get all associated left elements to a right element by its rowid
    pub fn get_associated_lefts_by_id(&self, right: i64) -> Vec<&L> {
        self.right_to_left
            .get(&right)
            .map(|l_ids| {
                l_ids
                    .iter()
                    .filter_map(|id| self.left_table.get(id))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Get all associated left elements to a right element
    pub fn get_associated_lefts(&self, right: &R) -> Vec<&L> {
        self.get_associated_lefts_by_id(right.rowid())
    }

    pub fn into_many_to_zero_left(self) -> ZeroToManyJoin<L, R>
    where
        R: Clone,
    {
        into_many_to_zero(self.left_table, self.right_table, self.right_to_left)
    }

    pub fn into_many_to_zero_right(self) -> ZeroToManyJoin<R, L>
    where
        L: Clone,
    {
        into_many_to_zero(self.right_table, self.left_table, self.left_to_right)
    }
}

fn into_many_to_zero<L, R>(
    left_table: Table<L>,
    right_table: Table<R>,
    right_to_left: HashMap<i64, Vec<i64>>,
) -> ZeroToManyJoin<L, R>
where
    L: HasRowID,
    R: HasRowID + Clone,
{
    let mut new_map = ZeroToManyJoin::default();

    new_map.insert_left(None);
    for left in left_table.into_iter() {
        new_map.insert_left(Some(left));
    }

    for right in right_table {
        let lefts = right_to_left
            .get(&right.rowid())
            .cloned()
            .unwrap_or_default();

        if lefts.is_empty() {
            new_map.push_entry(None, right);
        } else {
            lefts
                .into_iter()
                .for_each(|left_id| new_map.push_right_by_id(left_id, right.clone()));
        }
    }

    new_map
}

impl<L, R> Default for ManyToManyJoin<L, R> {
    fn default() -> Self {
        Self {
            left_table: Table::default(),
            right_table: Table::default(),
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }
}

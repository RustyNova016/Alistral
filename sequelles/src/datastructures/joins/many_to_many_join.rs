use std::collections::HashMap;

use crate::Table;
use crate::has_rowid::HasRowID;

/// Represent a Many to Many join in the database.
///
/// While it can be useful, prefer using [`ManyToZeroJoin`] or [`ZeroToManyJoin`] when possible, as they take less memory and promote less cloning
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

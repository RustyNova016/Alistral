use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;
use sqlx::FromRow;

use crate::RowId;

pub mod as_rowid_join;
pub mod index;
pub mod many_to_zero_join;
pub mod zero_to_many_join;

/// Represent a returned row during a many to many query.
#[derive(Clone, PartialEq, Eq, Hash, Debug, FromRow)]
pub struct JoinRelation<T> {
    /// The row ID of the entity having been queried
    pub original_id: i64,

    /// The associated entity
    #[sqlx(flatten)]
    pub data: T,
}

/// A collection of [`JoinRelation`]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JoinCollection<R> {
    joins: Vec<JoinRelation<R>>,
}

impl<R> JoinCollection<R> {
    /// Return a hashmap of <Rowid, T>. This allow of quick and hashless (On T) lookups.
    fn get_index<T>(data: Vec<T>) -> HashMap<i64, Vec<T>>
    where
        T: RowId,
    {
        data.into_iter()
            .map(|join| (join.get_row_id(), join))
            .into_group_map()
    }

    /// Convert the join into a hashmap. This implies that:
    /// - Right(self) has many Left
    /// - Left has one Right
    ///
    /// ex: a Recording (Right) has many Artists (Left)
    pub fn one_to_many<L>(self, left_values: Vec<L>) -> HashMap<R, Vec<L>>
    where
        L: RowId,
        R: Eq + Hash,
    {
        let mut left_index = Self::get_index(left_values);

        let mut output = HashMap::new();

        for (left_id, right) in self.joins.into_iter().map(|j| (j.original_id, j.data)) {
            let entry: &mut Vec<L> = output.entry(right).or_default();

            if let Some(lefts) = left_index.remove(&left_id) {
                entry.extend(lefts);
            }
        }
        output
    }

    pub fn many_to_many_right<L>(self, left_values: Vec<L>) -> HashMap<R, Vec<L>>
    where
        L: RowId + Clone,
        R: Eq + Hash,
    {
        let left_index = Self::get_index(left_values);
        let mut output = HashMap::new();
        for (left_id, right) in self.joins.into_iter().map(|j| (j.original_id, j.data)) {
            let entry: &mut Vec<L> = output.entry(right).or_default();

            if let Some(lefts) = left_index.get(&left_id) {
                entry.extend(lefts.to_owned());
            }
        }
        output
    }
}

impl<T> From<Vec<JoinRelation<T>>> for JoinCollection<T> {
    fn from(value: Vec<JoinRelation<T>>) -> Self {
        Self { joins: value }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Data(pub i64);

// impl Data {
//     pub fn new(data: i64) -> Self {
//         Self(data)
//     }

//     pub fn get_id(&self) -> i64 {
//         self.0
//     }

//     pub fn get_left_id(&self) -> i64 {
//         self.get_id() % 100
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Join(pub i64, pub Data);

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct JoinCollection(pub Vec<Join>);

// impl JoinCollection {
//     fn get_index(data: Vec<Data>) -> HashMap<i64, Vec<Data>> {
//         data.into_iter()
//             .map(|join| (join.get_id(), join))
//             .into_group_map()
//     }

//     pub fn one_to_many_naive(self, originals: Vec<Data>) -> HashMap<Data, Vec<Data>> {
//         let mut right_index = self
//             .0
//             .into_iter()
//             .map(|join| (join.0, join.1))
//             .into_group_map();

//         let mut output: HashMap<Data, Vec<Data>> = HashMap::new();
//         for left in originals {
//             let get_id = left.get_id();
//             let entry = output.entry(left).or_default();

//             if let Some(right) = right_index.remove(&get_id) {
//                 entry.extend(right);
//             }
//         }
//         output
//     }

//     pub fn many_to_one(self, originals: Vec<Data>) -> HashMap<Data, Vec<Data>> {
//         let mut left_index = Self::get_index(originals);

//         let mut output: HashMap<Data, Vec<Data>> = HashMap::new();
//         for (left_id, right) in self.0.into_iter().map(|j| (j.0, j.1)) {
//             let entry = output.entry(right).or_default();

//             if let Some(right) = left_index.remove(&left_id) {
//                 entry.extend(right);
//             }
//         }
//         output
//     }

//     pub fn many_to_many_right(self, originals: Vec<Data>) -> HashMap<Data, Vec<Data>> {
//         let left_index = Self::get_index(originals);

//         let mut output: HashMap<Data, Vec<Data>> = HashMap::new();
//         for (left_id, right) in self.0.into_iter().map(|j| (j.0, j.1)) {
//             let entry = output.entry(right).or_default();

//             if let Some(right) = left_index.get(&left_id) {
//                 entry.extend(right.clone());
//             }
//         }
//         output
//     }
// }

use std::hash::Hash;

use sqlx::FromRow;

use crate::ManyToManyJoin;
use crate::ManyToZeroJoin;
use crate::has_rowid::HasRowID;

/// Represent a returned row during a many to many query.
#[derive(Clone, PartialEq, Eq, Hash, Debug, FromRow)]
pub struct JoinRelation<T> {
    /// The row ID of the entity having been queried
    pub original_id: i64,

    /// The associated entity
    #[sqlx(flatten)]
    pub data: T,
}

impl<T> JoinRelation<T> {
    /// Convert the join relation into a tuple
    pub fn into_tupple(self) -> (i64, T) {
        (self.original_id, self.data)
    }
}

/// A collection of [`JoinRelation`]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JoinCollection<T> {
    joins: Vec<JoinRelation<T>>,
}

impl<R> JoinCollection<R> {
    pub fn len(&self) -> usize {
        self.joins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.joins.is_empty()
    }

    /// Convert the joins into a [`ManyToZeroJoin`]. This assumes that the current elements are the right elements of the join,
    /// and the provided ones are the left elements.
    pub fn into_many_to_zero<L, T>(self, left_elements: T) -> ManyToZeroJoin<L, R>
    where
        L: HasRowID,
        T: IntoIterator<Item = L>,
    {
        let mut smart_join = ManyToZeroJoin::default();

        // Insert the left values
        for left in left_elements {
            smart_join.insert(left, None);
        }

        // Now add the right values
        for (l_id, right) in self.joins.into_iter().map(|join| join.into_tupple()) {
            smart_join.replace_by_id(l_id, right);
        }

        smart_join
    }

    pub fn into_many_to_many<L, T>(self, left_elements: T) -> ManyToManyJoin<L, R>
    where
        L: HasRowID,
        R: HasRowID,
        T: IntoIterator<Item = L>,
    {
        let mut smart_join = ManyToManyJoin::default();

        // Insert the left values
        for left in left_elements {
            smart_join.add_left(left);
        }

        for (l_id, right) in self.joins.into_iter().map(|join| join.into_tupple()) {
            smart_join.add_relation_ids(l_id, right.rowid());
            smart_join.add_right(right);
        }

        smart_join
    }
}

impl<T> From<Vec<JoinRelation<T>>> for JoinCollection<T> {
    fn from(value: Vec<JoinRelation<T>>) -> Self {
        Self { joins: value }
    }
}

use core::hash::Hash;
use std::collections::HashMap;

use extend::ext;
use itertools::Itertools;
use sqlx::FromRow;

use crate::RowId;

pub fn inner_join_values<IdT, T, U, IteT, IteU>(left: IteT, mut right: IteU) -> Vec<(T, U)>
where
    IdT: Eq,
    U: Clone,
    IteT: Iterator<Item = (IdT, T)>,
    IteU: Iterator<Item = (IdT, U)>,
{
    left.filter_map(|(id_left, left_value)| {
        right
            .find(|(id_right, _)| id_left == *id_right)
            .map(|(_, right_value)| (left_value, right_value.clone()))
    })
    .collect_vec()
}

/// Represent a returned row during a many to many query.
#[derive(Clone, PartialEq, Eq, Hash, Debug, FromRow)]
pub struct JoinRelation<T, U> {
    /// The row ID of the entity having been queried
    pub original_id: T,

    /// The associated entity
    #[sqlx(flatten)]
    pub data: U,
}

/// A collection of [`JoinRelation`]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JoinCollection<T, U> {
    joins: Vec<JoinRelation<T, U>>,
}

impl<R> JoinCollection<i64, R>
where
    R: RowId + Eq + Hash,
{
    /// Associate the left entity's rowids to right entities
    pub fn into_lid_hashmap(self) -> HashMap<i64, Vec<R>> {
        self.joins
            .into_iter()
            .map(|join| (join.original_id, join.data))
            .into_group_map()
    }

    // /// Associate the right entity's rowids to left entities
    // pub fn into_rid_hashmap<L>(&self, left_entities: Vec<L>) -> HashMap<i64, Vec<L>>
    // where
    //     L: RowId,
    //     R: RowId
    // {
    //     for rid in self.joins.iter().map(|join| join.data.get_row_id()) {

    //     }
    // }

    /// Convert the join into a hashmap. This implies that:
    /// - Right has many Left
    /// - Left has one Right
    ///
    /// ex: a Release (Right) has many Tracks (Left)
    pub fn into_inner_many_to_one_hashmap<L>(self, left_entities: Vec<L>) -> HashMap<R, Vec<L>>
    where
        L: RowId,
    {
        // Construct an
        let mut l_index = left_entities
            .into_iter()
            .into_group_map_by(|ent| ent.get_row_id());

        let mut output = HashMap::new();
        for (l_id, right) in self.into_id_values() {
            let entry: &mut Vec<L> = output.entry(right).or_default();
            if let Some(left) = l_index.remove(&l_id) {
                entry.extend(left)
            }
        }

        output
    }

    fn into_id_values(self) -> impl Iterator<Item = (i64, R)> {
        self.joins
            .into_iter()
            .map(|join| (join.original_id, join.data))
    }
}

impl<LId, R: Clone> JoinCollection<LId, R> {

    pub fn into_hashmap<L, LIte, F>(
        self,
        left_elements: LIte,
        mut mapping: F,
    ) -> HashMap<LId, (L, Vec<R>)>
    where
        F: FnMut(&LId, &L) -> bool,
        LIte: IntoIterator<Item = L>,
        L: Clone,
        LId: Hash + Eq + Clone,
    {
        let mut output = HashMap::new();

        for left_element in left_elements {
            for join in &self.joins {
                // Is the join constraint valid?
                if mapping(&join.original_id, &left_element) {
                    // Yes! Add to output
                    output
                        .entry(join.original_id.clone())
                        .or_insert_with(|| (left_element.clone(), Vec::new()))
                        .1
                        .push(join.data.clone())
                }
            }
        }

        output
    }
}

impl<T, U> From<Vec<JoinRelation<T, U>>> for JoinCollection<T, U> {
    fn from(value: Vec<JoinRelation<T, U>>) -> Self {
        Self { joins: value }
    }
}

#[ext(name = InvertJoin)]
pub impl<L: Clone + RowId, R: RowId> HashMap<i64, (L, Vec<R>)> {
    fn invert_join(self) -> HashMap<i64, (R, Vec<L>)> {
        let mut new = HashMap::new();

        for (_, (left_element, right_elements)) in self {
            for right_element in right_elements {
                new.entry(right_element.get_row_id())
                    .or_insert_with(|| (right_element, Vec::new()))
                    .1
                    .push(left_element.clone())
            }
        }

        new
    }
}

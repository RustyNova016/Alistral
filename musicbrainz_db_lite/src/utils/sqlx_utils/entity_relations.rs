use core::hash::Hash;
use std::collections::HashMap;

use extend::ext;
use itertools::Itertools;
use sqlx::FromRow;

use crate::RowId;

#[derive(Clone, PartialEq, Eq, Hash)]
#[deprecated]
pub struct EntityRelations<T, U> {
    pub relations: Vec<(T, U)>,
}

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

#[derive(Clone, PartialEq, Eq, Hash, Debug, FromRow)]
pub struct JoinRelation<T, U> {
    pub original_id: T,
    #[sqlx(flatten)]
    pub data: U,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JoinCollection<T, U> {
    joins: Vec<JoinRelation<T, U>>,
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

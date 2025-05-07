use std::collections::HashMap;
use std::hash::Hash;

use crate::RowId;
use crate::utils::sqlx_utils::joins::as_rowid_join::AsRowIdJoin;
use crate::utils::sqlx_utils::joins::index::Index;
use crate::utils::sqlx_utils::joins::index::IntoIndexes;
use crate::utils::sqlx_utils::joins::simple_join::OneToManyRightJoin;

/// An hashmap that represent a `LEFT JOIN`, where an element of the Left table <u>can</u> have <u>one</u> element of the Right table
///
/// Example: **a Listen <u>can</u> have a Recording**, but a Recording can have <u>many</u> Listens
pub struct ManyToZeroLeftJoin<L, R>(pub(super) HashMap<L, Option<R>>);

impl<L, R> ManyToZeroLeftJoin<L, R> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn invert_with_ids(self) -> ZeroToManyJoin<R, L>
    where
        L: RowId,
        R: RowId,
    {
        let join = self.as_rowid_join().invert_many();
        let (left_index, right_index) = self.into_indexes();

        todo!()
    }

    /// Invert the join into a `OneToManyRightJoin<R, L>`. This should be used for light values that implement Hash and Clone like ids
    pub fn invert_many(self) -> ZeroToManyJoin<R, L>
    where
        R: Eq + Hash,
    {
        let mut new_map: HashMap<Option<R>, Vec<L>> = HashMap::new();

        for (left, right) in self.0.into_iter() {
            new_map.entry(right).or_default().push(left)
        }

        OneToManyRightJoin(new_map)
    }
}

impl<L, R> AsRowIdJoin for ManyToZeroLeftJoin<L, R>
where
    L: RowId,
    R: RowId,
{
    type Output = ManyToZeroLeftJoin<i64, i64>;

    fn as_rowid_join(&self) -> Self::Output {
        let mut join = ManyToZeroLeftJoin::new();

        for (left, right) in &self.0 {
            let left = left.get_row_id();
            let right = right.as_ref().map(|r| r.get_row_id());
            join.0.insert(left, right);
        }

        join
    }
}

impl<L, R> IntoIndexes<L, R> for ManyToZeroLeftJoin<L, R>
where
    L: RowId,
    R: RowId,
{
    fn into_indexes(self) -> (super::index::Index<L>, super::index::Index<R>) {
        let mut left_index = Index::new();
        let mut right_index = Index::new();

        for (left, right) in self.0.into_iter() {
            left_index.insert(left);
            if let Some(right) = right {
                right_index.insert(right);
            }
        }

        (left_index, right_index)
    }
}

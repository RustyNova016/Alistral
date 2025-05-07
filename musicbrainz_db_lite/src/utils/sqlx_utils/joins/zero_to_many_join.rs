use std::collections::HashMap;

use crate::RowId;
use crate::utils::sqlx_utils::joins::index::ApplyIndexes;
use crate::utils::sqlx_utils::joins::index::Index;

/// An hashmap that represent a `RIGHT JOIN`, where an element of the Left table <u>can</u> have <u>multiples</u> elements of the Right table.
/// The Left element can also be missing
pub struct ZeroToManyJoin<L, R>(pub(super) HashMap<Option<L>, Vec<R>>);

impl<L, R> Default for ZeroToManyJoin<L, R> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

// impl<L, R> ApplyIndexes<L, R> for ZeroToManyJoin<i64, i64>
// where
//     L: RowId,
//     R: RowId,
// {
//     fn apply_indexes(self, mut left_index: Index<L>, mut right_index: Index<R>) {
//         let mut new_map = ZeroToManyJoin::default();
//         for (left, rights) in self.0 {
//             let left = match left {
//                 Some(id) => left_index.remove(id),
//                 None => None,
//             };

//             new_map.entry()

//             for right in rights {
//                 let right = 
//             }
//         }

//         new_map
//     }
// }

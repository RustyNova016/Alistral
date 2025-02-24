use std::collections::HashMap;

use crate::RowId;

/// a Map like struct that holds two entities (**L**eft and **R**ight)
#[derive(Debug, Default)]
pub struct JoinMap<L: RowId, R>(HashMap<i64, (L, HashMap<i64, R>)>);

impl<L: RowId, R> JoinMap<L, R> {
    pub fn get_by_id(&self, id: i64) -> Option<&(L, HashMap<i64, R>)> {
        self.0.get(&id)
    }

    // pub fn insert(&mut self, id_left: i64, value: R) -> Option<(L, HashMap<i64, R>)> {
    //     self.0.insert(id_left, value)
    // }

    pub fn invert_join(self) -> JoinMap<R, L>
    where
        L: Clone,
        R: RowId,
    {
        let mut new = HashMap::new();

        for (left_id, (left_element, right_elements)) in self.0 {
            for right_element in right_elements {
                new.entry(right_element.1.get_row_id())
                    .or_insert_with(|| (right_element.1, HashMap::new()))
                    .1
                    .insert(left_id, left_element.clone());
            }
        }

        JoinMap(new)
    }

    // pub fn merge_joins<T>(self, other: JoinMap<R, T>) -> JoinMap<L, JoinMap<R, T>>
    // where
    //     R: RowId,
    // {
    //     let mut new = HashMap::new();

    //     let asociations = self.0.into_iter().flat_map(|(_, (left, rights))| {
    //         rights.into_iter().map(|(_, right)| {

    //         })
    //     });

    //     for (id_left, (left, right_elements)) in self.0 {
    //         for (_, right) in right_elements {
    //             let thirds = other.get_by_id(right.get_row_id());

    //             new.entry(left.get_row_id())
    //                 .or_insert_with(|| (left, JoinMap::default())).1.insert(id_left, );
    //         }
    //     }

    //     JoinMap(new)
    // }
}

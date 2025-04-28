use std::collections::HashMap;

use crate::models::shared_traits::HasMBID;

pub struct RelationAssertion<'l> {
    pub left_id: &'l str,
    pub right_id: &'l str,
}

impl RelationAssertion<'_> {
    pub fn assert_has_element_with_mbid<T: HasMBID>(&self, items: &[T]) {
        if items.iter().all(|i| i.get_mbid() != self.right_id) {
            panic!(
                "Assertion `has_element_with_mbid` failed. All elements provided lacks the MBID \"{}\"",
                self.right_id
            );
        }
    }

    pub fn assert_batch_join_has_relation<T, U>(
        assertions: &[Self],
        results: &HashMap<i64, (&&T, Vec<U>)>,
    ) where
        T: HasMBID,
        U: HasMBID,
    {
        for (left, right) in results.values() {
            let Some(assertion) = assertions.iter().find(|a| a.left_id == left.get_mbid()) else {
                continue;
            };

            assertion.assert_has_element_with_mbid(right);
        }
    }
}

pub fn assert_has_element_with_mbid<T: HasMBID>(items: &[T], mbid: &str) {
    if items.iter().all(|i| i.get_mbid() != mbid) {
        panic!(
            "Assertion `has_element_with_mbid` failed. All elements provided lacks the MBID \"{mbid}\""
        );
    }
}

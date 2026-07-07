use core::hash::Hash;
use std::collections::HashMap;

use itertools::Itertools;

pub trait HashJoin<T> {
    type Output;

    fn hash_join(self, other: T) -> Self::Output;
}

impl<'l, Key, Data1, Data2> HashJoin<&'l HashMap<Key, Data2>> for &'l HashMap<Key, Data1>
where
    Key: Hash + Eq,
{
    type Output = HashMap<&'l Key, (Option<&'l Data1>, Option<&'l Data2>)>;

    fn hash_join(self, other: &'l HashMap<Key, Data2>) -> Self::Output {
        let keys = self.keys().chain(other.keys()).unique();

        let mut result = HashMap::new();
        for key in keys {
            result.insert(key, (self.get(key), other.get(key)));
        }

        result
    }
}

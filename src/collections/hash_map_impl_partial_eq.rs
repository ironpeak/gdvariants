use std::{
    self,
    hash::{BuildHasher, Hash},
};

use crate::collections::HashMap;

impl<K, V, S> PartialEq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        self.base.eq(&other.base)
    }
}

use std::hash::{BuildHasher, Hash};

use crate::collections::HashSet;

impl<T, S> PartialEq for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &HashSet<T, S>) -> bool {
        self.base.eq(&other.base)
    }
}

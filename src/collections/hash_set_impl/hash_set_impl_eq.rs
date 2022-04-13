use std::hash::{BuildHasher, Hash};

use crate::collections::HashSet;

impl<T, S> Eq for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

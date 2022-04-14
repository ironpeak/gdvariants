use std::hash::{BuildHasher, Hash};

use crate::collections::HashSet;

impl<T, S> FromIterator<T> for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> HashSet<T, S> {
        let mut set = HashSet::with_hasher(S::default());
        set.extend(iter);
        set
    }
}

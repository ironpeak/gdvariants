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

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_from_iter() {
        let stdset: std::collections::HashSet<i32> =
            std::collections::HashSet::from_iter([2, 1, 3]);
        let crateset: HashSet<i32> = HashSet::from_iter([2, 1, 3]);

        assert_eq!(stdset, crateset);
    }
}

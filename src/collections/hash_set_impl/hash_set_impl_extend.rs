use std::hash::{BuildHasher, Hash};

use crate::collections::HashSet;

impl<T, S> Extend<T> for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.base.extend(iter);
    }
}

impl<'a, T, S> Extend<&'a T> for HashSet<T, S>
where
    T: 'a + Eq + Hash + Copy,
    S: BuildHasher,
{
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.base.extend(iter.into_iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_extend() {
        let mut stdset = std::collections::HashSet::from([2, 1, 3]);
        let mut crateset = HashSet::from([2, 1, 3]);

        stdset.extend([1, 2, 3]);
        crateset.extend([1, 2, 3]);

        assert_eq!(stdset, crateset);
    }
}

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

impl<T, S> PartialEq<std::collections::HashSet<T, S>> for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &std::collections::HashSet<T, S>) -> bool {
        self.base.eq(other)
    }
}

impl<T, S> PartialEq<HashSet<T, S>> for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &HashSet<T, S>) -> bool {
        self.eq(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_eq_a() {
        let vec_a = HashSet::from([2, 1, 3]);
        let vec_b = HashSet::from([2, 1, 3]);

        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn test_eq_b() {
        let vec_a = HashSet::from([2, 1, 3]);
        let vec_b = std::collections::HashSet::from([2, 1, 3]);

        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn test_eq_c() {
        let vec_a = std::collections::HashSet::from([2, 1, 3]);
        let vec_b = HashSet::from([2, 1, 3]);

        assert_eq!(vec_a, vec_b);
    }
}

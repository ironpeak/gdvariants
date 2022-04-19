use std::{collections::hash_map::RandomState, hash::Hash};

use crate::collections::HashSet;

impl<T, const N: usize> From<[T; N]> for HashSet<T, RandomState>
where
    T: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashSet;
    ///
    /// let set1 = HashSet::from([1, 2, 3, 4]);
    /// let set2: HashSet<_> = [1, 2, 3, 4].into();
    /// assert_eq!(set1, set2);
    /// ```
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_from_array() {
        let source = [2, 1, 3];

        let stdvec = std::collections::HashSet::from(source);
        let cratevec = HashSet::from(source);

        assert_eq!(stdvec, cratevec);
    }
}

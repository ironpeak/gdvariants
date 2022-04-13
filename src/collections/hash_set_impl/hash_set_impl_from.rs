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

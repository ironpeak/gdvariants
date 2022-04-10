use std::{self, collections::hash_map::RandomState, hash::Hash};

use crate::collections::HashMap;

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState>
where
    K: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashMap;
    ///
    /// let map1 = HashMap::from([(1, 2), (3, 4)]);
    /// let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    #[inline]
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

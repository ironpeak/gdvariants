use std::{
    self,
    collections::hash_map::{IntoIter, Iter, IterMut},
};

use crate::collections::HashMap;

impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.base.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.base.iter_mut()
    }
}

impl<K, V, S> IntoIterator for HashMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    /// Creates a consuming iterator, that is, one that moves each key-value
    /// pair out of the map in arbitrary order. The map cannot be used after
    /// calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashMap;
    ///
    /// let map = HashMap::from([
    ///     ("a", 1),
    ///     ("b", 2),
    ///     ("c", 3),
    /// ]);
    ///
    /// // Not possible with .iter()
    /// let vec: Vec<(&str, i32)> = map.into_iter().collect();
    /// ```
    #[inline]
    fn into_iter(self) -> IntoIter<K, V> {
        self.base.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_into_iterator() {
        let stdmap = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let cratemap = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        let stdres: HashMap<i32, i32> = stdmap.into_iter().collect();
        let crateres: HashMap<i32, i32> = cratemap.into_iter().collect();

        assert_eq!(stdres, crateres);
    }
}

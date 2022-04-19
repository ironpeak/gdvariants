use std::collections::hash_set::{IntoIter, Iter};

use crate::collections::HashSet;

impl<T, S> IntoIterator for HashSet<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out
    /// of the set in arbitrary order. The set cannot be used after calling
    /// this.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashSet;
    /// let mut set = HashSet::new();
    /// set.insert("a".to_string());
    /// set.insert("b".to_string());
    ///
    /// // Not possible to collect to a Vec<String> with a regular `.iter()`.
    /// let v: Vec<String> = set.into_iter().collect();
    ///
    /// // Will print in an arbitrary order.
    /// for x in &v {
    ///     println!("{}", x);
    /// }
    /// ```
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        self.base.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a HashSet<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Iter<'a, T> {
        self.base.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_into_iterator() {
        let stdvec = std::collections::HashSet::from([2, 1, 3]);
        let cratevec = HashSet::from([2, 1, 3]);

        assert_eq!(
            stdvec.into_iter().collect::<HashSet<i32>>(),
            cratevec.into_iter().collect::<HashSet<i32>>()
        );
    }
}

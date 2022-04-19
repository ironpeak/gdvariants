use std::{
    hash::{BuildHasher, Hash},
    ops::BitAnd,
};

use crate::collections::HashSet;

impl<T, S> BitAnd<&HashSet<T, S>> for &HashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = HashSet<T, S>;

    /// Returns the intersection of `self` and `rhs` as a new `HashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashSet;
    ///
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([2, 3, 4]);
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        HashSet {
            base: self.base.bitand(&rhs.base),
        }
    }
}

impl<T, S> BitAnd<&std::collections::HashSet<T, S>> for &HashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = HashSet<T, S>;

    /// Returns the intersection of `self` and `rhs` as a new `HashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashSet;
    ///
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([2, 3, 4]);
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &std::collections::HashSet<T, S>) -> HashSet<T, S> {
        HashSet {
            base: self.base.bitand(rhs),
        }
    }
}

impl<T, S> BitAnd<&HashSet<T, S>> for &std::collections::HashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = HashSet<T, S>;
    /// Returns the intersection of `self` and `rhs` as a new `HashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gdvariants::collections::HashSet;
    ///
    /// let a = HashSet::from([1, 2, 3]);
    /// let b = HashSet::from([2, 3, 4]);
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        HashSet {
            base: self.bitand(&rhs.base),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_a() {
        let set_a = HashSet::from([2, 1, 3]);
        let set_b = HashSet::from([1, 5, 3]);

        assert_eq!(&set_a & &set_b, HashSet::from([1, 3]));
    }

    #[test]
    fn test_b() {
        let set_a = std::collections::HashSet::from([2, 1, 3]);
        let set_b = HashSet::from([1, 5, 3]);

        assert_eq!(&set_a & &set_b, HashSet::from([1, 3]));
    }

    #[test]
    fn test_c() {
        let set_a = HashSet::from([2, 1, 3]);
        let set_b = std::collections::HashSet::from([1, 5, 3]);

        assert_eq!(&set_a & &set_b, HashSet::from([1, 3]));
    }
}

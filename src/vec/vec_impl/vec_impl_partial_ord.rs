use std::cmp::Ordering;

use crate::vec::Vec;

/// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
impl<T> PartialOrd for Vec<T>
where
    T: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.base.partial_cmp(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_partial_cmp() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        assert_eq!(stdvec.partial_cmp(&stdvec), cratevec.partial_cmp(&cratevec));
    }
}

use std::cmp::Ordering;

use crate::vec::Vec;

/// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
impl<T> Ord for Vec<T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Vec<T>) -> Ordering {
        self.base.cmp(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_cmp() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        assert_eq!(stdvec.cmp(&stdvec), cratevec.cmp(&cratevec));
    }
}

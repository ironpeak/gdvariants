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

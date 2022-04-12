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

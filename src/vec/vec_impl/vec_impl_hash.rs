use crate::vec::Vec;
use std::hash::{Hash, Hasher};

impl<T> Hash for Vec<T>
where
    T: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state)
    }
}

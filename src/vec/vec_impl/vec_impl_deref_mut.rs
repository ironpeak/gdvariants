use crate::vec::Vec;
use std::ops::DerefMut;

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.base.deref_mut()
    }
}

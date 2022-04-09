use crate::vec::Vec;
use std::ops::Deref;

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.base.deref()
    }
}

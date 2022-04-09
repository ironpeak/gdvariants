use crate::vec::Vec;

use std::borrow::Borrow;

impl<T> Borrow<[T]> for Vec<T> {
    fn borrow(&self) -> &[T] {
        self.base.borrow()
    }
}

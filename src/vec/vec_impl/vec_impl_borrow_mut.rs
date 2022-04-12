use crate::vec::Vec;

use std::borrow::BorrowMut;

impl<T> BorrowMut<[T]> for Vec<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.base.borrow_mut()
    }
}

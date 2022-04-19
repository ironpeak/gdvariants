use crate::vec::Vec;

use std::borrow::BorrowMut;

impl<T> BorrowMut<[T]> for Vec<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.base.borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::vec::Vec;

    #[test]
    fn test_borrow_mut() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &[i32] = stdvec.borrow_mut();
        let crateres: &[i32] = cratevec.borrow_mut();

        assert_eq!(stdres, crateres);
    }
}

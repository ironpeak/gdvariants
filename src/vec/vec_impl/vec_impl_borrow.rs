use crate::vec::Vec;

use std::borrow::Borrow;

impl<T> Borrow<[T]> for Vec<T> {
    fn borrow(&self) -> &[T] {
        self.base.borrow()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::vec::Vec;

    #[test]
    fn test_borrow() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &[i32] = stdvec.borrow();
        let crateres: &[i32] = cratevec.borrow();

        assert_eq!(stdres, crateres);
    }
}

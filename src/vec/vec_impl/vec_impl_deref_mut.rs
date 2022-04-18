use crate::vec::Vec;
use std::ops::DerefMut;

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.base.deref_mut()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use crate::vec::Vec;

    #[test]
    fn test_deref_mut_slice() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        let stdres = stdvec.deref_mut();
        let crateres = cratevec.deref_mut();

        assert_eq!(stdres, crateres);
    }
}

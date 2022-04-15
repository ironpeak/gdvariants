use crate::vec::Vec;
use std::ops::Deref;

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.base.deref()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::vec::Vec;

    #[test]
    fn test_deref_slice() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres = stdvec.deref();
        let crateres = cratevec.deref();

        assert_eq!(stdres, crateres);
    }
}

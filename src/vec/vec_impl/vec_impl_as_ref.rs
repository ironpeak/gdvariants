use crate::vec::Vec;

impl<T> AsRef<Vec<T>> for Vec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self
    }
}

impl<T> AsRef<[T]> for Vec<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_as_ref() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &std::vec::Vec<i32> = stdvec.as_ref();
        let crateres: &Vec<i32> = cratevec.as_ref();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_as_ref_slice() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &[i32] = stdvec.as_ref();
        let crateres: &[i32] = cratevec.as_ref();

        assert_eq!(stdres, crateres);
    }
}

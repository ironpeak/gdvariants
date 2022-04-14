use crate::vec::Vec;

impl<T> AsMut<Vec<T>> for Vec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self
    }
}

impl<T> AsMut<[T]> for Vec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_as_mut_vec() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &std::vec::Vec<i32> = stdvec.as_mut();
        let crateres: &Vec<i32> = cratevec.as_mut();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_as_mut_slice() {
        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: &[i32] = stdvec.as_mut();
        let crateres: &[i32] = cratevec.as_mut();

        assert_eq!(stdres, crateres);
    }
}

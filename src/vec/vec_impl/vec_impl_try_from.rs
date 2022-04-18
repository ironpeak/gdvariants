use crate::vec::Vec;

impl<T, const N: usize> TryFrom<Vec<T>> for [T; N] {
    type Error = Vec<T>;

    fn try_from(vec: Vec<T>) -> Result<[T; N], Vec<T>> {
        let result: Result<[T; N], std::vec::Vec<T>> = vec.base.try_into();
        match result {
            Ok(array) => Ok(array),
            Err(vec) => Err(Vec { base: vec }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_try_from_ok() {
        let stdvec: Result<[i32; 3], std::vec::Vec<i32>> = <[i32; 3]>::try_from(vec![2, 1, 3]);
        let cratevec: Result<[i32; 3], Vec<i32>> = <[i32; 3]>::try_from(Vec::from(vec![2, 1, 3]));

        assert_eq!(stdvec.unwrap(), cratevec.unwrap());
    }

    #[test]
    fn test_try_from_err() {
        let stdvec: Result<[i32; 2], std::vec::Vec<i32>> = <[i32; 2]>::try_from(vec![2, 1, 3]);
        let cratevec: Result<[i32; 2], Vec<i32>> = <[i32; 2]>::try_from(Vec::from(vec![2, 1, 3]));

        assert_eq!(stdvec.unwrap_err(), cratevec.unwrap_err());
    }
}

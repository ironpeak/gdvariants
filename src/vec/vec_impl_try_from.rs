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

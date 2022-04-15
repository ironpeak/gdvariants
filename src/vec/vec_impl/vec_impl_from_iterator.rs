use crate::vec::Vec;

impl<T> FromIterator<T> for Vec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut vec = Vec::new();
        vec.extend(iter);
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_from_iter() {
        let stdvec = std::vec::Vec::from_iter([2, 1, 3]);
        let cratevec = Vec::from_iter([2, 1, 3]);

        assert_eq!(stdvec, cratevec);
    }
}

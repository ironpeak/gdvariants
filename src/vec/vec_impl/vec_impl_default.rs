use crate::vec::Vec;

impl<T> Default for Vec<T> {
    /// Creates an empty `Vec<T>`.
    fn default() -> Vec<T> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_default_vec() {
        let stdvec: std::vec::Vec<i32> = std::vec::Vec::default();
        let cratevec: Vec<i32> = Vec::default();

        assert_eq!(stdvec, cratevec);
    }
}

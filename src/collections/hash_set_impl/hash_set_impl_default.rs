use crate::collections::HashSet;

impl<T, S> Default for HashSet<T, S>
where
    S: Default,
{
    /// Creates an empty `HashSet<T, S>` with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashSet<T, S> {
        HashSet {
            base: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_default() {
        let stdvec: std::collections::HashSet<i32> = std::collections::HashSet::default();
        let cratevec: HashSet<i32> = HashSet::default();

        assert_eq!(stdvec, cratevec);
    }
}

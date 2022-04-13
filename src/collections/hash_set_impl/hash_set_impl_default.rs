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

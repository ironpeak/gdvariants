use std::{ self};

use crate::collections::HashMap;

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap {
            base: std::collections::HashMap::default(),
        }
    }
}

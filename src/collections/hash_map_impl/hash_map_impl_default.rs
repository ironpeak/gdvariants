use std::{self};

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

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_default() {
        let stdmap: std::collections::HashMap<i32, i32> = std::collections::HashMap::default();
        let cratemap: HashMap<i32, i32> = HashMap::default();

        assert_eq!(stdmap, cratemap);
    }
}

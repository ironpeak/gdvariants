use std::{
    self,
    borrow::Borrow,
    hash::{BuildHasher, Hash},
    ops::Index,
};

use crate::collections::HashMap;

impl<K, Q: ?Sized, V, S> Index<&'_ Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.base.index(key)
    }

    type Output = V;
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::collections::HashMap;

    #[test]
    fn test_index() {
        let stdmap = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let cratemap = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        let stdres = stdmap.index(&1);
        let crateres = cratemap.index(&1);

        assert_eq!(stdres, crateres);
    }
}

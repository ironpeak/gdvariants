use std::{
    self,
    hash::{BuildHasher, Hash},
};

use crate::collections::HashMap;

impl<K, V, S> PartialEq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        self.base.eq(&other.base)
    }
}

impl<K, V, S> PartialEq<std::collections::HashMap<K, V, S>> for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &std::collections::HashMap<K, V, S>) -> bool {
        self.base.eq(other)
    }
}

impl<K, V, S> PartialEq<HashMap<K, V, S>> for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        self.eq(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_eq_a() {
        let vec_a = HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let vec_b = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn test_eq_b() {
        let vec_a = HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let vec_b = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);

        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn test_eq_c() {
        let vec_a = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let vec_b = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        assert_eq!(vec_a, vec_b);
    }
}

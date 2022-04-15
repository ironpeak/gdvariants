use crate::vec::Vec;
use std::hash::{Hash, Hasher};

impl<T> Hash for Vec<T>
where
    T: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use crate::vec::Vec;

    #[test]
    fn test_hash() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let mut stdhasher = DefaultHasher::new();
        stdvec.hash(&mut stdhasher);

        let mut cratehasher = DefaultHasher::new();
        cratevec.hash(&mut cratehasher);

        assert_eq!(stdhasher.finish(), cratehasher.finish());
    }
}

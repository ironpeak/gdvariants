use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize};

use crate::collections::HashSet;

impl<'de, T> Deserialize<'de> for HashSet<T>
where
    T: Eq + Hash + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HashSet {
            base: std::collections::HashSet::deserialize::<D>(deserializer)?,
        })
    }
}

impl<T> Serialize for HashSet<T>
where
    T: Eq + Hash + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        std::collections::HashSet::serialize(&self.base, serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_from_str() {
        let json = "[2, 1, 3]".to_string();

        let stdset: std::collections::HashSet<i32> = serde_json::from_str(&json).unwrap();
        let crateset: HashSet<i32> = serde_json::from_str(&json).unwrap();

        assert_eq!(stdset, crateset);
    }

    #[test]
    fn test_to_string() {
        let stdset = std::collections::HashSet::from([2, 1, 3]);
        let crateset = HashSet::from([2, 1, 3]);

        let stdres: String = serde_json::to_string(&stdset).unwrap();
        let crateres: String = serde_json::to_string(&crateset).unwrap();

        assert_eq!(stdres.len(), crateres.len());
    }
}

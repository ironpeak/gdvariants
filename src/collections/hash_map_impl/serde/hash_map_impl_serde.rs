use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize};

use crate::collections::HashMap;

impl<'de, K, V> Deserialize<'de> for HashMap<K, V>
where
    K: Eq + Hash + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(HashMap {
            base: std::collections::HashMap::deserialize::<D>(deserializer)?,
        })
    }
}

impl<K, V> Serialize for HashMap<K, V>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        std::collections::HashMap::serialize(&self.base, serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_from_str() {
        let json = "{\"2\": 4, \"1\": 2, \"3\": 6}".to_string();

        let stdvec: std::collections::HashMap<String, i32> = serde_json::from_str(&json).unwrap();
        let cratevec: HashMap<String, i32> = serde_json::from_str(&json).unwrap();

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_to_string() {
        let stdvec = std::collections::HashMap::from([
            ("2".to_string(), 4),
            ("1".to_string(), 2),
            ("3".to_string(), 6),
        ]);
        let cratevec = HashMap::from([
            ("2".to_string(), 4),
            ("1".to_string(), 2),
            ("3".to_string(), 6),
        ]);

        let stdres: String = serde_json::to_string(&stdvec).unwrap();
        let crateres: String = serde_json::to_string(&cratevec).unwrap();

        assert_eq!(stdres.len(), crateres.len());
    }
}

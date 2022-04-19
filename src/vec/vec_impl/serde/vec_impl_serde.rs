use serde::{Deserialize, Deserializer, Serialize};

use crate::vec::Vec;

impl<'de, T> Deserialize<'de> for Vec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec {
            base: std::vec::Vec::deserialize::<D>(deserializer)?,
        })
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        std::vec::Vec::serialize(&self.base, serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_from_str() {
        let json = "[2, 1, 3]".to_string();

        let stdvec: std::vec::Vec<i32> = serde_json::from_str(&json).unwrap();
        let cratevec: Vec<i32> = serde_json::from_str(&json).unwrap();

        assert_eq!(stdvec, cratevec);
    }

    #[test]
    fn test_to_string() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: String = serde_json::to_string(&stdvec).unwrap();
        let crateres: String = serde_json::to_string(&cratevec).unwrap();

        assert_eq!(stdres, crateres);
    }
}

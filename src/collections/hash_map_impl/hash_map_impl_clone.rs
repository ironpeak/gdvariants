use crate::collections::HashMap;

impl<K, V, S> Clone for HashMap<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        HashMap {
            base: self.base.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.base.clone_from(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_clone() {
        let stdmap = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let cratemap = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        let stdres: std::collections::HashMap<i32, i32> = stdmap.clone();
        let crateres: HashMap<i32, i32> = cratemap.clone();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_clone_from() {
        let stdsource = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let cratesource = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        let mut stdmap = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let mut cratemap = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        stdmap.clone_from(&stdsource);
        cratemap.clone_from(&cratesource);

        assert_eq!(stdmap, cratemap);
    }
}

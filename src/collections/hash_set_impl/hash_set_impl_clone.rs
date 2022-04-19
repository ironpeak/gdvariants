use crate::collections::HashSet;

impl<T, S> Clone for HashSet<T, S>
where
    T: Clone,
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.base.clone_from(&other.base);
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_clone() {
        let stdset = std::collections::HashSet::from([2, 1, 3]);
        let crateset = HashSet::from([2, 1, 3]);

        let stdres: std::collections::HashSet<i32> = stdset.clone();
        let crateres: HashSet<i32> = crateset.clone();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_clone_from() {
        let stdsource = std::collections::HashSet::from([3, 2, 4]);
        let cratesource = HashSet::from([3, 2, 4]);

        let mut stdset = std::collections::HashSet::from([2, 1, 3]);
        let mut crateset = HashSet::from([2, 1, 3]);

        stdset.clone_from(&stdsource);
        crateset.clone_from(&cratesource);

        assert_eq!(stdset, crateset);
    }
}

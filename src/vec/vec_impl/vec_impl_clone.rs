use crate::vec::Vec;

impl<T> Clone for Vec<T>
where
    T: Clone,
{
    fn clone(&self) -> Vec<T> {
        Vec {
            base: self.base.clone(),
        }
    }

    fn clone_from(&mut self, other: &Vec<T>) {
        self.base.clone_from(&other.base)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_clone() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres: std::vec::Vec<i32> = stdvec.clone();
        let crateres: Vec<i32> = cratevec.clone();

        assert_eq!(stdres, crateres);
    }

    #[test]
    fn test_clone_from() {
        let stdsource = vec![3, 2, 4];
        let cratesource = Vec::from(vec![3, 2, 4]);

        let mut stdvec = vec![2, 1, 3];
        let mut cratevec = Vec::from(vec![2, 1, 3]);

        stdvec.clone_from(&stdsource);
        cratevec.clone_from(&cratesource);

        assert_eq!(stdvec, cratevec);
    }
}

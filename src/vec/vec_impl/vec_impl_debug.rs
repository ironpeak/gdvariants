use crate::vec::Vec;
use std::fmt::{Debug, Error, Formatter};

impl<T> Debug for Vec<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.base.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec;

    #[test]
    fn test_debug() {
        let stdvec = vec![2, 1, 3];
        let cratevec = Vec::from(vec![2, 1, 3]);

        let stdres = format!("{:?}", &stdvec);
        let crateres = format!("{:?}", &cratevec);

        assert_eq!(stdres, crateres);
    }
}

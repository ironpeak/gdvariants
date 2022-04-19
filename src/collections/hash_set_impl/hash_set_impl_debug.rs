use std::fmt::{Debug, Formatter, Result};

use crate::collections::HashSet;

impl<T, S> Debug for HashSet<T, S>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.base.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashSet;

    #[test]
    fn test_clone() {
        let stdset = std::collections::HashSet::from([2, 1, 3]);
        let crateset = HashSet::from([2, 1, 3]);

        let stdres: String = format!("{:?}", &stdset);
        let crateres: String = format!("{:?}", &crateset);

        assert_eq!(stdres.len(), crateres.len());
    }
}

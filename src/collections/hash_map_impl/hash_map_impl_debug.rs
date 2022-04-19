use std::{self, fmt::Debug};

use crate::collections::HashMap;

impl<K, V, S> Debug for HashMap<K, V, S>
where
    K: Debug,
    V: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HashMap;

    #[test]
    fn test_debug() {
        let stdmap = std::collections::HashMap::from([(2, 4), (1, 2), (3, 6)]);
        let cratemap = HashMap::from([(2, 4), (1, 2), (3, 6)]);

        let stdres = format!("{:?}", &stdmap);
        let crateres = format!("{:?}", &cratemap);

        assert_eq!(stdres.len(), crateres.len());
    }
}

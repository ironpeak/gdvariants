use std::fmt::{Debug, Formatter, Result};

use crate::collections::HashSet;

impl<T, S> Debug for HashSet<T, S>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

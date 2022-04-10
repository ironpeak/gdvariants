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

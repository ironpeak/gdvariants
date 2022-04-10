use std::{
    self,
    hash::{BuildHasher, Hash},
};

use crate::collections::HashMap;

impl<K, V, S> Eq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

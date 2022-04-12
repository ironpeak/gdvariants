use std::{
    borrow::Borrow,
    collections::{
        hash_map::RandomState,
        hash_set::{Difference, Drain, Intersection, IntoIter, Iter, SymmetricDifference, Union},
        TryReserveError,
    },
    fmt,
    hash::{BuildHasher, Hash},
    ops::{BitAnd, BitOr, BitXor, Sub},
};

use crate::collections::HashSet;

impl<T, S> PartialEq for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &HashSet<T, S>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|key| other.contains(key))
    }
}

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

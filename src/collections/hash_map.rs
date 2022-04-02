use std;
use std::borrow::*;
use std::collections::hash_map::*;
use std::collections::*;
use std::fmt::Debug;
use std::hash::*;
use std::ops::*;

use gdnative::core_types::Dictionary;
use gdnative::prelude::*;

pub struct HashMap<K, V, S = RandomState> {
    hash_map: std::collections::HashMap<K, V, S>,
}

impl<K, V, S> FromVariant for HashMap<K, V, S>
where
    K: Hash + Eq + FromVariant,
    V: FromVariant,
    S: BuildHasher + Default,
{
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dictionary = Dictionary::from_variant(variant)?;
        let mut hash_map: HashMap<K, V, S> =
            HashMap::with_capacity_and_hasher(dictionary.len() as usize, Default::default());
        for (variant_key, variant_value) in dictionary.iter() {
            let key = K::from_variant(&variant_key)?;
            let value = V::from_variant(&variant_value)?;
            hash_map.insert(key, value);
        }
        Ok(hash_map)
    }
}

impl<K, V, S> ToVariant for HashMap<K, V, S>
where
    K: ToVariantEq + ToVariant,
    V: ToVariant,
{
    fn to_variant(&self) -> Variant {
        let dictionary = Dictionary::new();
        for (key, value) in &self.hash_map {
            dictionary.insert(key, value);
        }
        dictionary.owned_to_variant()
    }
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            hash_map: std::collections::HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        HashMap {
            hash_map: std::collections::HashMap::with_capacity(capacity),
        }
    }
}

impl<K, V, S> HashMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap {
            hash_map: std::collections::HashMap::with_hasher(hash_builder),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> HashMap<K, V, S> {
        HashMap {
            hash_map: std::collections::HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    pub fn capacity(&self) -> usize {
        self.hash_map.capacity()
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        self.hash_map.keys()
    }

    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.hash_map.into_keys()
    }

    pub fn values(&self) -> Values<'_, K, V> {
        self.hash_map.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.hash_map.values_mut()
    }

    pub fn into_values(self) -> IntoValues<K, V> {
        self.hash_map.into_values()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.hash_map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.hash_map.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        self.hash_map.drain()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.hash_map.retain::<F>(f)
    }

    pub fn clear(&mut self) {
        self.hash_map.clear()
    }

    pub fn hasher(&self) -> &S {
        self.hash_map.hasher()
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn reserve(&mut self, additional: usize) {
        self.hash_map.reserve(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.hash_map.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.hash_map.shrink_to_fit()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.hash_map.shrink_to(min_capacity)
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.hash_map.entry(key)
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.get(k)
    }

    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.get_key_value(k)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.contains_key(k)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.get_mut(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.hash_map.insert(k, v)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.remove(k)
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.hash_map.remove_entry(k)
    }
}

impl<K, V, S> Clone for HashMap<K, V, S>
where
    K: Clone,
    V: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        HashMap {
            hash_map: self.hash_map.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.hash_map.clone_from(&other.hash_map)
    }
}

impl<K, V, S> PartialEq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        self.hash_map.eq(&other.hash_map)
    }
}

impl<K, V, S> Eq for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

impl<K, V, S> Debug for HashMap<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.hash_map.fmt(f)
    }
}

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    fn default() -> HashMap<K, V, S> {
        HashMap {
            hash_map: std::collections::HashMap::default(),
        }
    }
}

impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &'_ Q) -> &Self::Output {
        self.hash_map.index(index)
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState>
where
    K: Eq + Hash,
{
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.hash_map.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> IterMut<'a, K, V> {
        self.hash_map.iter_mut()
    }
}

impl<K, V, S> IntoIterator for HashMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        self.hash_map.into_iter()
    }
}

impl<K, V, S> FromIterator<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        map
    }
}

impl<K, V, S> Extend<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.hash_map.extend(iter)
    }
}

// TODO
// impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S>
// where
//     K: Eq + Hash + Copy,
//     V: Copy,
//     S: BuildHasher,
// {
//     fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
//         self.hash_map.extend(iter)
//     }
// }

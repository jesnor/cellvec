use crate::{clone_cell::CloneCell, safe_traits::SafeTraits};
use std::borrow::Borrow;
use std::collections::hash_map::{IntoKeys, IntoValues, RandomState};
use std::collections::{HashMap, TryReserveError};
use std::hash::{BuildHasher, Hash};

pub struct CloneMap<K: SafeTraits, V: SafeTraits, S: SafeTraits = RandomState>(CloneCell<HashMap<K, V, S>>);

impl<K: SafeTraits, V: SafeTraits> CloneMap<K, V, RandomState> {
    pub fn new() -> Self { Self(HashMap::new().into()) }
    pub fn with_capacity(capacity: usize) -> Self { HashMap::with_capacity(capacity).into() }
}

impl<K: SafeTraits, V: SafeTraits, S: SafeTraits> CloneMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> Self { HashMap::with_hasher(hash_builder).into() }

    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        HashMap::with_capacity_and_hasher(capacity, hasher).into()
    }

    pub fn capacity(&self) -> usize { unsafe { self.0.get_ref().capacity() } }
    pub fn into_keys(self) -> IntoKeys<K, V> { self.0.into_inner().into_keys() }
    pub fn into_values(self) -> IntoValues<K, V> { self.0.into_inner().into_values() }
    pub fn clear(&self) { unsafe { (*self.0.get()).clear() } }
    pub fn len(&self) -> usize { unsafe { self.0.get_ref().len() } }
    pub fn is_empty(&self) -> bool { unsafe { self.0.get_ref().is_empty() } }
}

impl<K: SafeTraits, V: SafeTraits, S: Clone + SafeTraits> CloneMap<K, V, S> {
    pub fn hasher(&self) -> S { unsafe { self.0.get_ref().hasher().clone() } }
}

impl<K, V, S> CloneMap<K, V, S>
where
    K: Eq + Hash + SafeTraits,
    V: SafeTraits,
    S: BuildHasher + SafeTraits,
{
    pub fn reserve(&self, additional: usize) { unsafe { (*self.0.get()).reserve(additional) } }

    pub fn try_reserve(&self, additional: usize) -> Result<(), TryReserveError> {
        unsafe { (*self.0.get()).try_reserve(additional) }
    }

    pub fn shrink_to_fit(&self) { unsafe { (*self.0.get()).shrink_to_fit() } }
    pub fn shrink_to(&self, min_capacity: usize) { unsafe { (*self.0.get()).shrink_to(min_capacity) } }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unsafe { self.0.get_ref().contains_key(k) }
    }

    pub fn insert(&self, k: K, v: V) -> Option<V> { unsafe { (*self.0.get()).insert(k, v) } }

    pub fn remove<Q>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unsafe { (*self.0.get()).remove(k) }
    }

    pub fn remove_entry<Q>(&self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unsafe { (*self.0.get()).remove_entry(k) }
    }
}

impl<K, V, S> CloneMap<K, V, S>
where
    K: Eq + Hash + SafeTraits,
    V: Clone + SafeTraits,
    S: BuildHasher + SafeTraits,
{
    pub fn get<Q>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unsafe { self.0.get_ref().get(k).cloned() }
    }
}

impl<K, V, S> CloneMap<K, V, S>
where
    K: Clone + Eq + Hash + SafeTraits,
    V: Clone + SafeTraits,
    S: BuildHasher + SafeTraits,
{
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        unsafe { self.0.get_ref().get_key_value(k).map(|(k, v)| (k.clone(), v.clone())) }
    }
}

impl<K: SafeTraits, V: SafeTraits> Default for CloneMap<K, V, RandomState> {
    fn default() -> Self { Self::new() }
}

impl<K: SafeTraits, V: SafeTraits, S: SafeTraits> From<HashMap<K, V, S>> for CloneMap<K, V, S> {
    fn from(m: HashMap<K, V, S>) -> Self { Self(m.into()) }
}

impl<K: Eq + Hash + SafeTraits, V: PartialEq + SafeTraits, S: BuildHasher + SafeTraits> PartialEq
    for CloneMap<K, V, S>
{
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<K: Eq + Hash + SafeTraits, V: Eq + SafeTraits, S: BuildHasher + SafeTraits> Eq for CloneMap<K, V, S> {}

impl<K: Clone + SafeTraits, V: Clone + SafeTraits, S: Clone + SafeTraits> Clone for CloneMap<K, V, S> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

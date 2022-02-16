use std::fmt::Debug;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

pub struct Ptr<T> {
    value: T,
}

impl<T> Ptr<T> {
    pub fn new(value: T) -> Self { Self { value } }
    pub fn set(&mut self, value: T) { self.value = value; }
}

impl<T: Copy> Ptr<T> {
    pub fn get(&self) -> T { self.value }
}

impl<'t, T> PartialEq for Ptr<&'t T> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.value as *const T, other.value as *const T) }
}

impl<'t, T> PartialEq for Ptr<&'t mut T> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.value as *const T, other.value as *const T) }
}

impl<'t, T> Eq for Ptr<&'t T> {}
impl<'t, T> Eq for Ptr<&'t mut T> {}

impl<'t, T> Hash for Ptr<&'t T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { (self.value as *const T).hash(state); }
}

impl<'t, T> Hash for Ptr<&'t mut T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { (self.value as *const T).hash(state); }
}

impl<'t, T> PartialOrd for Ptr<&'t T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.value as *const T).partial_cmp(&(other.value as *const T))
    }
}

impl<'t, T> PartialOrd for Ptr<&'t mut T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.value as *const T).partial_cmp(&(other.value as *const T))
    }
}

impl<'t, T> Ord for Ptr<&'t T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.value as *const T).cmp(&(other.value as *const T)) }
}

impl<'t, T> Ord for Ptr<&'t mut T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.value as *const T).cmp(&(other.value as *const T)) }
}

impl<T: Deref> Deref for Ptr<T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target { self.value.deref() }
}

impl<T: DerefMut> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.value.deref_mut() }
}

impl<T: Clone> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: Copy> Copy for Ptr<T> {}

impl<T> Debug for Ptr<&T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ptr").field("value", &(self.value as *const T)).finish()
    }
}

impl<T> From<T> for Ptr<T> {
    fn from(value: T) -> Self { Ptr::new(value) }
}

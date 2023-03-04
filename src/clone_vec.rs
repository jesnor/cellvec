use crate::{clone_cell::CloneCell, safe_traits::SafeTraits};
use std::{collections::TryReserveError, io::Write, slice::SliceIndex};

pub struct CloneVec<T: SafeTraits>(CloneCell<Vec<T>>);

impl<T: SafeTraits> CloneVec<T> {
    pub fn new() -> Self { Self(Vec::new().into()) }
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity).into()) }
    pub fn len(&self) -> usize { unsafe { self.0.get_ref().len() } }
    pub fn capacity(&self) -> usize { unsafe { self.0.get_ref().capacity() } }
    pub fn clear(&self) { unsafe { (*self.0.get()).clear() } }
    pub fn is_empty(&self) -> bool { unsafe { self.0.get_ref().is_empty() } }
    pub fn insert(&self, index: usize, element: T) { unsafe { (*self.0.get()).insert(index, element) } }
    pub fn into_boxed_slice(self) -> Box<[T]> { self.0.into_inner().into_boxed_slice() }
    pub fn leak<'a>(self) -> &'a mut [T] { self.0.into_inner().leak() }
    pub fn pop(&self) -> Option<T> { unsafe { (*self.0.get()).pop() } }
    pub fn push(&self, value: T) { unsafe { (*self.0.get()).push(value) } }
    pub fn remove(&self, index: usize) -> T { unsafe { (*self.0.get()).remove(index) } }
    pub fn reserve(&self, additional: usize) { unsafe { (*self.0.get()).reserve(additional) } }
    pub fn reserve_exact(&self, additional: usize) { unsafe { (*self.0.get()).reserve_exact(additional) } }
    pub fn swap_remove(&self, index: usize) -> T { unsafe { (*self.0.get()).swap_remove(index) } }
    pub fn shrink_to(&self, min_capacity: usize) { unsafe { (*self.0.get()).shrink_to(min_capacity) } }
    pub fn shrink_to_fit(&self) { unsafe { (*self.0.get()).shrink_to_fit() } }
    pub fn truncate(&self, len: usize) { unsafe { (*self.0.get()).truncate(len) } }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn set_len(&self, new_len: usize) { unsafe { (*self.0.get()).set_len(new_len) } }

    pub fn try_reserve(&self, additional: usize) -> Result<(), TryReserveError> {
        unsafe { (*self.0.get()).try_reserve(additional) }
    }

    pub fn try_reserve_exact(&self, additional: usize) -> Result<(), TryReserveError> {
        unsafe { (*self.0.get()).try_reserve_exact(additional) }
    }

    pub fn resize_with<F>(&self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        unsafe { (*self.0.get()).resize_with(new_len, f) }
    }

    pub fn get<I>(&self, index: I) -> Option<<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
        I::Output: Clone,
    {
        unsafe { self.0.get_ref().get(index).cloned() }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
        I::Output: Clone,
    {
        self.0.get_ref().get_unchecked(index).clone()
    }
}

impl<T: Clone + SafeTraits> CloneVec<T> {
    pub fn extend_from_slice(&self, other: &[T]) { unsafe { (*self.0.get()).extend_from_slice(other) } }
    pub fn resize(&self, new_len: usize, value: T) { unsafe { (*self.0.get()).resize(new_len, value) } }

    pub fn retain<F>(&self, mut f: F)
    where
        F: FnMut(T) -> bool,
    {
        unsafe { (*self.0.get()).retain(|v| f(v.clone())) }
    }

    pub fn iter(&self) -> ClonedVecIter<'_, T> { ClonedVecIter { vec: self, index: 0 } }
}

impl<T: SafeTraits> Default for CloneVec<T> {
    fn default() -> Self { Self::new() }
}

impl<T: Clone + SafeTraits> Clone for CloneVec<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: std::fmt::Debug + SafeTraits> std::fmt::Debug for CloneVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CloneVec").field(unsafe { self.0.get_ref() }).finish()
    }
}

impl<T: PartialEq<U> + SafeTraits, U: SafeTraits> PartialEq<CloneVec<U>> for CloneVec<T> {
    fn eq(&self, other: &CloneVec<U>) -> bool { unsafe { self.0.get_ref() == other.0.get_ref() } }
}

impl<T: Eq + SafeTraits> Eq for CloneVec<T> {}

impl<T: PartialOrd<T> + SafeTraits> PartialOrd<CloneVec<T>> for CloneVec<T> {
    fn partial_cmp(&self, other: &CloneVec<T>) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) }
}

impl<T: Ord + SafeTraits> Ord for CloneVec<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}

impl<T: std::hash::Hash + SafeTraits> std::hash::Hash for CloneVec<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl Write for CloneVec<u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { unsafe { (*self.0.get()).write(buf) } }
    fn flush(&mut self) -> std::io::Result<()> { unsafe { (*self.0.get()).flush() } }
}

pub struct ClonedVecIter<'t, T: SafeTraits> {
    vec:   &'t CloneVec<T>,
    index: usize,
}

impl<'t, T: Clone + SafeTraits> Iterator for ClonedVecIter<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.vec.get(self.index);
        self.index += 1;
        r
    }
}

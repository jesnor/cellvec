use crate::{
    clone_cell::CloneCell,
    safe_traits::SafeTraits,
    vec_cell_trait::{VecCellEntryImpl, VecCellIter, VecCellTrait, VecCellEntryIter},
};
use std::{collections::TryReserveError, io::Write, slice::SliceIndex};

pub struct VecCell<T: SafeTraits>(CloneCell<Vec<T>>);

impl<T: SafeTraits> VecCell<T> {
    #[inline(always)]
    unsafe fn inner(&self) -> &Vec<T> { self.0.get_ref() }

    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    unsafe fn inner_mut(&self) -> &mut Vec<T> { &mut *self.0.get() }
}

impl<T: SafeTraits> VecCellTrait<T> for VecCell<T> {
    fn new() -> Self { Self(Vec::new().into()) }

    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity).into()) }

    #[inline(always)]
    fn len(&self) -> usize { unsafe { self.inner().len() } }

    #[inline(always)]
    fn capacity(&self) -> usize { unsafe { self.inner().capacity() } }

    #[inline(always)]
    fn clear(&self) { unsafe { self.inner_mut().clear() } }

    #[inline(always)]
    fn is_empty(&self) -> bool { unsafe { self.inner().is_empty() } }

    #[inline(always)]
    fn insert(&self, index: usize, element: T) { unsafe { self.inner_mut().insert(index, element) } }

    #[inline(always)]
    fn into_boxed_slice(self) -> Box<[T]> { self.0.into_inner().into_boxed_slice() }

    #[inline(always)]
    fn leak<'a>(self) -> &'a mut [T] { self.0.into_inner().leak() }

    #[inline(always)]
    fn pop(&self) -> Option<T> { unsafe { self.inner_mut().pop() } }

    #[inline(always)]
    fn push(&self, value: T) { unsafe { self.inner_mut().push(value) } }

    #[inline(always)]
    fn remove(&self, index: usize) -> T { unsafe { self.inner_mut().remove(index) } }

    #[inline(always)]
    fn reserve(&self, additional: usize) { unsafe { self.inner_mut().reserve(additional) } }

    #[inline(always)]
    fn reserve_exact(&self, additional: usize) { unsafe { self.inner_mut().reserve_exact(additional) } }

    #[inline(always)]
    fn swap_remove(&self, index: usize) -> T { unsafe { self.inner_mut().swap_remove(index) } }

    #[inline(always)]
    fn shrink_to(&self, min_capacity: usize) { unsafe { self.inner_mut().shrink_to(min_capacity) } }

    #[inline(always)]
    fn shrink_to_fit(&self) { unsafe { self.inner_mut().shrink_to_fit() } }

    #[inline(always)]
    fn truncate(&self, len: usize) { unsafe { self.inner_mut().truncate(len) } }

    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_len(&self, new_len: usize) { unsafe { self.inner_mut().set_len(new_len) } }

    #[inline(always)]
    fn try_reserve(&self, additional: usize) -> Result<(), TryReserveError> {
        unsafe { self.inner_mut().try_reserve(additional) }
    }

    #[inline(always)]
    fn try_reserve_exact(&self, additional: usize) -> Result<(), TryReserveError> {
        unsafe { self.inner_mut().try_reserve_exact(additional) }
    }

    #[inline(always)]
    fn resize_with<F: FnMut() -> T>(&self, new_len: usize, f: F) { unsafe { self.inner_mut().resize_with(new_len, f) } }

    #[inline(always)]
    fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<<I as SliceIndex<[T]>>::Output>
    where
        I::Output: Clone,
    {
        unsafe { self.inner().get(index).cloned() }
    }

    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn get_unchecked<I: SliceIndex<[T]>>(&self, index: I) -> <I as SliceIndex<[T]>>::Output
    where
        I::Output: Clone,
    {
        self.inner().get_unchecked(index).clone()
    }

    #[inline(always)]
    fn set<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T) {
        unsafe {
            *self.inner_mut()
                .get_mut(index).unwrap() = v;
        }
    }

    #[inline(always)]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_unchecked<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T) {
        *self.inner_mut().get_unchecked_mut(index) = v;
    }

    type Iter<'t> = VecCellIter<'t, Self, T> where Self: 't, T: Clone;

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_>
    where
        T: Clone,
    {
        VecCellIter::new(self)
    }

    #[inline(always)]
    fn extend_from_slice(&self, other: &[T])
    where
        T: Clone,
    {
        unsafe { self.inner_mut().extend_from_slice(other) }
    }

    #[inline(always)]
    fn resize(&self, new_len: usize, value: T)
    where
        T: Clone,
    {
        unsafe { self.inner_mut().resize(new_len, value) }
    }

    #[inline(always)]
    fn retain<F: FnMut(T) -> bool>(&self, mut f: F)
    where
        T: Clone,
    {
        unsafe { self.inner_mut().retain(|v| f(v.clone())) }
    }

    type Entry<'t> = VecCellEntryImpl<'t, Self, T> where Self: 't;

    #[inline(always)]
    fn entry(&self, index: usize) -> Self::Entry<'_>
    where
        T: Clone,
    {
        Self::Entry::new(self, index)
    }

    type EntryIter<'t> = VecCellEntryIter<'t, Self, T> 
    where
        Self: 't,
        T: Clone;

        #[inline(always)]
        fn entries(&self) -> Self::EntryIter<'_>
    where
        T: Clone {
            Self::EntryIter::new(self)
        }
}

impl<T: SafeTraits> Default for VecCell<T> {
    #[inline(always)]
    fn default() -> Self { Self::new() }
}

impl<T: Clone + SafeTraits> Clone for VecCell<T> {
    #[inline(always)]
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: std::fmt::Debug + SafeTraits> std::fmt::Debug for VecCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CloneVec").field(unsafe { self.inner() }).finish()
    }
}

impl<T: PartialEq<U> + SafeTraits, U: SafeTraits> PartialEq<VecCell<U>> for VecCell<T> {
    #[inline(always)]
    fn eq(&self, other: &VecCell<U>) -> bool { self.0 == other.0 }
}

impl<T: Eq + SafeTraits> Eq for VecCell<T> {}

impl<T: PartialOrd<T> + SafeTraits> PartialOrd<VecCell<T>> for VecCell<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &VecCell<T>) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) }
}

impl<T: Ord + SafeTraits> Ord for VecCell<T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}

impl<T: std::hash::Hash + SafeTraits> std::hash::Hash for VecCell<T> {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl Write for VecCell<u8> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { unsafe { self.inner_mut().write(buf) } }

    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> { unsafe { self.inner_mut().flush() } }
}

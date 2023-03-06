use crate::{
    safe_traits::SafeTraits,
    vec_cell_trait::{VecCellIter, VecCellTrait, VecCellEntryImpl, VecCellEntryIter},
};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::TryReserveError,
    io::Write,
    ops::Deref,
    slice::SliceIndex,
};

pub struct VecRefCell<T: SafeTraits>(RefCell<Vec<T>>);

impl<T: SafeTraits> VecRefCell<T> {
    fn inner(&self) -> Ref<'_, Vec<T>> { self.0.borrow() }
    fn inner_mut(&self) -> RefMut<'_, Vec<T>> { self.0.borrow_mut() }
}

impl<T: SafeTraits> VecCellTrait<T> for VecRefCell<T> {
    fn new() -> Self { Self(Vec::new().into()) }
    fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity).into()) }
    fn len(&self) -> usize { self.inner().len() }
    fn capacity(&self) -> usize { self.inner().capacity() }
    fn clear(&self) { self.inner_mut().clear() }
    fn is_empty(&self) -> bool { self.inner().is_empty() }
    fn insert(&self, index: usize, element: T) { self.inner_mut().insert(index, element) }
    fn into_boxed_slice(self) -> Box<[T]> { self.0.into_inner().into_boxed_slice() }
    fn leak<'a>(self) -> &'a mut [T] { self.0.into_inner().leak() }
    fn pop(&self) -> Option<T> { self.inner_mut().pop() }
    fn push(&self, value: T) { self.inner_mut().push(value) }
    fn remove(&self, index: usize) -> T { self.inner_mut().remove(index) }
    fn reserve(&self, additional: usize) { self.inner_mut().reserve(additional) }
    fn reserve_exact(&self, additional: usize) { self.inner_mut().reserve_exact(additional) }
    fn swap_remove(&self, index: usize) -> T { self.inner_mut().swap_remove(index) }
    fn shrink_to(&self, min_capacity: usize) { self.inner_mut().shrink_to(min_capacity) }
    fn shrink_to_fit(&self) { self.inner_mut().shrink_to_fit() }
    fn truncate(&self, len: usize) { self.inner_mut().truncate(len) }
    unsafe fn set_len(&self, new_len: usize) { self.inner_mut().set_len(new_len) }

    fn try_reserve(&self, additional: usize) -> Result<(), TryReserveError> { self.inner_mut().try_reserve(additional) }

    fn try_reserve_exact(&self, additional: usize) -> Result<(), TryReserveError> {
        self.inner_mut().try_reserve_exact(additional)
    }

    fn resize_with<F: FnMut() -> T>(&self, new_len: usize, f: F) { self.inner_mut().resize_with(new_len, f) }

    fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<<I as SliceIndex<[T]>>::Output>
    where
        I::Output: Clone,
    {
        self.inner().get(index).cloned()
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn get_unchecked<I: SliceIndex<[T]>>(&self, index: I) -> <I as SliceIndex<[T]>>::Output
    where
        I::Output: Clone,
    {
        self.inner().get_unchecked(index).clone()
    }

    fn set<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T) { *self.inner_mut().get_mut(index).unwrap() = v }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_unchecked<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T) {
        *self.inner_mut().get_unchecked_mut(index) = v;
    }

    type Iter<'t> = VecCellIter<'t, Self, T> where Self: 't, T: Clone;

    fn iter(&self) -> Self::Iter<'_>
    where
        T: Clone,
    {
        VecCellIter::new(self)
    }

    fn extend_from_slice(&self, other: &[T])
    where
        T: Clone,
    {
        self.inner_mut().extend_from_slice(other)
    }

    fn resize(&self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.inner_mut().resize(new_len, value)
    }

    fn retain<F: FnMut(T) -> bool>(&self, mut f: F)
    where
        T: Clone,
    {
        self.inner_mut().retain(|v| f(v.clone()))
    }

    type Entry<'t> = VecCellEntryImpl<'t, Self, T> where Self: 't;

    fn entry(&self, index: usize) -> Self::Entry<'_>
    where
        T: Clone,
    {
        Self::Entry::new(self, index, self.get(index).unwrap())
    }

    type EntryIter<'t> = VecCellEntryIter<'t, Self, T> 
    where
        Self: 't,
        T: Clone;

    fn entries(&self) -> Self::EntryIter<'_>
    where
        T: Clone {
            Self::EntryIter::new(self)
        }
}

impl<T: SafeTraits> Default for VecRefCell<T> {
    fn default() -> Self { Self::new() }
}

impl<T: Clone + SafeTraits> Clone for VecRefCell<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: std::fmt::Debug + SafeTraits> std::fmt::Debug for VecRefCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CloneVec").field(self.inner().deref()).finish()
    }
}

impl<T: PartialEq<U> + SafeTraits, U: SafeTraits> PartialEq<VecRefCell<U>> for VecRefCell<T> {
    fn eq(&self, other: &VecRefCell<U>) -> bool { self.inner().deref() == other.inner().deref() }
}

impl<T: Eq + SafeTraits> Eq for VecRefCell<T> {}

impl<T: PartialOrd<T> + SafeTraits> PartialOrd<VecRefCell<T>> for VecRefCell<T> {
    fn partial_cmp(&self, other: &VecRefCell<T>) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) }
}

impl<T: Ord + SafeTraits> Ord for VecRefCell<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}

impl<T: std::hash::Hash + SafeTraits> std::hash::Hash for VecRefCell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner().hash(state); }
}

impl Write for VecRefCell<u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.inner_mut().write(buf) }
    fn flush(&mut self) -> std::io::Result<()> { self.inner_mut().flush() }
}

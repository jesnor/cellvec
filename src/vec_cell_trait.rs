use crate::var::Var;
use std::{collections::TryReserveError, marker::PhantomData, slice::SliceIndex};

pub trait VecCellEntry<'t, T>: Var<T> {
    fn index(&self) -> usize;
    fn remove(&self) -> T;
}

pub trait VecCellTrait<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn clear(&self);
    fn is_empty(&self) -> bool;
    fn insert(&self, index: usize, element: T);
    fn into_boxed_slice(self) -> Box<[T]>;
    fn leak<'a>(self) -> &'a mut [T];
    fn pop(&self) -> Option<T>;
    fn push(&self, value: T);
    fn remove(&self, index: usize) -> T;
    fn reserve(&self, additional: usize);
    fn reserve_exact(&self, additional: usize);
    fn swap_remove(&self, index: usize) -> T;
    fn shrink_to(&self, min_capacity: usize);
    fn shrink_to_fit(&self);
    fn truncate(&self, len: usize);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_len(&self, new_len: usize);

    fn try_reserve(&self, additional: usize) -> Result<(), TryReserveError>;
    fn try_reserve_exact(&self, additional: usize) -> Result<(), TryReserveError>;

    fn resize_with<F: FnMut() -> T>(&self, new_len: usize, f: F);

    fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<<I as SliceIndex<[T]>>::Output>
    where
        I::Output: Clone;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn get_unchecked<I: SliceIndex<[T]>>(&self, index: I) -> <I as SliceIndex<[T]>>::Output
    where
        I::Output: Clone;

    fn set<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn set_unchecked<I: SliceIndex<[T], Output = T>>(&self, index: I, v: T);

    fn extend_from_slice(&self, other: &[T])
    where
        T: Clone;

    fn resize(&self, new_len: usize, value: T)
    where
        T: Clone;

    fn retain<F: FnMut(T) -> bool>(&self, f: F)
    where
        T: Clone;

    type Iter<'t>: Iterator<Item = T> + 't
    where
        Self: 't,
        T: Clone;

    fn iter(&self) -> Self::Iter<'_>
    where
        T: Clone;

    type Entry<'t>: VecCellEntry<'t, T>
    where
        Self: 't;

    fn entry(&self, index: usize) -> Self::Entry<'_>
    where
        T: Clone;

    type EntryIter<'t>: Iterator<Item = Self::Entry<'t>> + 't
    where
        Self: 't,
        T: Clone;

    fn entries(&self) -> Self::EntryIter<'_>
    where
        T: Clone;
}

pub struct VecCellIter<'t, V, T> {
    vec:      &'t V,
    index:    usize,
    _phantom: PhantomData<T>,
}

impl<'t, V, T> VecCellIter<'t, V, T> {
    pub(crate) fn new(vec: &'t V) -> Self {
        Self {
            vec,
            index: 0,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'t, T: Clone, V: VecCellTrait<T>> Iterator for VecCellIter<'t, V, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.vec.get(self.index);
        self.index += 1;
        r
    }
}

pub struct VecCellEntryImpl<'t, V, T> {
    vec:      &'t V,
    index:    usize,
    _phantom: PhantomData<T>,
}

impl<'t, V: VecCellTrait<T>, T> VecCellEntryImpl<'t, V, T> {
    pub fn new(vec: &'t V, index: usize) -> Self {
        Self {
            vec,
            index,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'t, V: VecCellTrait<T>, T> Var<T> for VecCellEntryImpl<'t, V, T> {
    fn set(&self, value: T) { self.vec.set(self.index, value) }

    fn get(&self) -> T
    where
        T: Clone,
    {
        self.vec.get(self.index).unwrap()
    }
}

impl<'t, V: VecCellTrait<T>, T> VecCellEntry<'t, T> for VecCellEntryImpl<'t, V, T> {
    fn index(&self) -> usize { self.index }
    fn remove(&self) -> T { self.vec.remove(self.index) }
}

pub struct VecCellEntryIter<'t, V, T> {
    vec:      &'t V,
    index:    usize,
    _phantom: PhantomData<T>,
}

impl<'t, V, T> VecCellEntryIter<'t, V, T> {
    pub(crate) fn new(vec: &'t V) -> Self {
        Self {
            vec,
            index: 0,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'t, T: Clone, V: VecCellTrait<T>> Iterator for VecCellEntryIter<'t, V, T> {
    type Item = VecCellEntryImpl<'t, V, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let r = VecCellEntryImpl {
                vec:      self.vec,
                index:    self.index,
                _phantom: PhantomData::default(),
            };

            self.index += 1;
            Some(r)
        } else {
            None
        }
    }
}

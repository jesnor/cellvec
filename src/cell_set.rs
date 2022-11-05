use crate::mcell::MCell;
use crate::refs::WeakRefTrait;
use crate::{cell_trait::CellTrait, clear::Clear};
use std::{cell::Cell, fmt::Debug, marker::PhantomData};

pub type Slot<T> = Cell<Option<T>>;

pub struct CellSet<T, A> {
    slots:    A,
    first:    MCell<usize>,
    _phantom: PhantomData<fn(T) -> T>,
}

pub type VecCellSet<T> = CellSet<T, Vec<Slot<T>>>;
pub type ArrayCellSet<T, const CAP: usize> = CellSet<T, [Slot<T>; CAP]>;

impl<T> CellSet<T, Vec<Slot<T>>> {
    pub fn new_vec(cap: usize) -> Self {
        let mut v = Vec::with_capacity(cap);
        v.resize_with(v.capacity(), Default::default);
        Self::new(v)
    }
}

impl<T, A: AsRef<[Slot<T>]>> CellSet<T, A> {
    #[must_use]
    pub fn new(slots: A) -> Self {
        let l = slots.as_ref().len();

        Self {
            slots,
            first: l.into(),
            _phantom: Default::default(),
        }
    }

    #[must_use]
    pub fn capacity(&self) -> usize { self.slots.as_ref().len() }

    #[must_use]
    pub fn len(&self) -> usize { self.capacity() - self.first.get() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.first.get() == self.capacity() }

    pub fn remove(&self, index: usize) -> T {
        let first = self.first.get();
        let slot = &self.slots.as_ref()[first + index];
        let elem = unsafe { slot.take().unwrap_unchecked() };
        slot.set(unsafe { self.slots.as_ref().get_unchecked(first) }.take());
        self.first.set(first + 1);
        elem
    }

    #[must_use]
    pub fn insert(&self, elem: T) -> Option<usize> {
        if self.len() >= self.capacity() {
            return None;
        }

        let index = self.first.get() - 1;
        self.first.set(index);
        let slot = &self.slots.as_ref()[index];
        slot.set(Some(elem));
        Some(0)
    }

    pub fn retain<F>(&self, f: impl Fn(&T) -> bool) {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.as_ref().get_unchecked(i) };
            let v = unsafe { c.take().unwrap_unchecked() };

            if !f(&v) {
                self.remove(i);
            } else {
                c.set(Some(v));
            }
        }
    }
}

impl<T: Copy, A: AsRef<[Slot<T>]>> CellSet<T, A> {
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> {
        self.slots.as_ref().get(self.first.get() + index).and_then(|s| s.get())
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ { self.slots.as_ref().iter().filter_map(|c| c.get()) }
}

impl<T: Clone, A: AsRef<[Slot<T>]>> CellSet<T, A> {
    #[must_use]
    pub fn get_clone(&self, index: usize) -> Option<T> {
        self.slots.as_ref().get(self.first.get() + index).and_then(|s| s.get_clone())
    }

    pub fn iter_clone(&self) -> impl Iterator<Item = T> + '_ {
        self.slots.as_ref()[self.first.get()..].iter().filter_map(|c| c.get_clone())
    }
}

impl<T: WeakRefTrait + Clone, A: AsRef<[Slot<T>]>> CellSet<T, A> {
    pub fn iter_ref(&self) -> impl Iterator<Item = T::Strong> + '_ { self.iter_clone().filter_map(|r| r.upgrade()) }
}

impl<T: PartialEq, A: AsRef<[Slot<T>]>> CellSet<T, A> {
    pub fn remove_first(&self, elem: T) -> Option<usize> {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.as_ref().get_unchecked(i) };
            let v = unsafe { c.take().unwrap_unchecked() };

            if v == elem {
                self.remove(i);
                return Some(i);
            } else {
                c.set(Some(v));
            }
        }

        None
    }
}

// impl<T: Clone, A: AsRef<[Slot<T>]>> Clone for CellSet<T, A> {
//     #[must_use]
//     fn clone(&self) -> Self {
//         let mut slots = Vec::with_capacity(self.capacity());
//         slots.resize_with(self.first.get(), Default::default);

//         for i in self.first.get()..self.capacity() {
//             let c = unsafe { self.slots.as_ref().get_unchecked(i) };
//             let e = unsafe { c.take().unwrap_unchecked() };
//             c.set(Some(e.clone()));
//             slots.push(Cell::new(Some(e)));
//         }

//         Self {
//             slots,
//             first: self.first.clone(),
//         }
//     }
// }

impl<T, A: AsRef<[Slot<T>]>> Clear for CellSet<T, A> {
    fn clear(&self) {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.as_ref().get_unchecked(i) };
            c.set(None);
        }

        self.first.set(self.capacity());
    }
}

impl<T, A: AsRef<[Slot<T>]>> Debug for CellSet<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("CellSet").finish() }
}

impl<T, A: AsRef<[Slot<T>]> + Default> Default for CellSet<T, A> {
    fn default() -> Self { Self::new(A::default()) }
}

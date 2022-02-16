use crate::clear::Clear;
use std::{cell::Cell, fmt::Debug};

type Slot<T> = Cell<Option<T>>;

pub struct CellSet<T> {
    slots: Vec<Slot<T>>,
    first: Cell<usize>,
}

impl<T> CellSet<T> {
    #[must_use]
    pub fn new(cap: usize) -> Self {
        let mut s = Self {
            slots: Vec::with_capacity(cap),
            first: Cell::new(cap),
        };

        s.slots.resize_with(cap, Default::default);
        s
    }

    #[must_use]
    pub fn capacity(&self) -> usize { self.slots.len() }

    #[must_use]
    pub fn len(&self) -> usize { self.slots.len() - self.first.get() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.first.get() == self.slots.len() }

    pub fn remove(&self, index: usize) -> T {
        let first = self.first.get();
        let slot = &self.slots[first + index];
        let elem = unsafe { slot.take().unwrap_unchecked() };
        slot.set(unsafe { self.slots.get_unchecked(first) }.take());
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
        let slot = &self.slots[index];
        slot.set(Some(elem));
        Some(0)
    }

    pub fn retain<F>(&self, f: impl Fn(&T) -> bool) {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.get_unchecked(i) };
            let v = unsafe { c.take().unwrap_unchecked() };

            if !f(&v) {
                self.remove(i);
            } else {
                c.set(Some(v));
            }
        }
    }
}

impl<T: Copy> CellSet<T> {
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> { self.slots.get(self.first.get() + index).and_then(|s| s.get()) }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ { self.slots.iter().filter_map(|c| c.get()) }
}

impl<T: Clone> CellSet<T> {
    #[must_use]
    pub fn get_clone(&self, index: usize) -> Option<T> {
        self.slots.get(self.first.get() + index).and_then(|s| {
            let v = s.take();
            s.set(v.clone());
            v
        })
    }

    pub fn iter_clone(&self) -> impl Iterator<Item = T> + '_ {
        self.slots[self.first.get()..].iter().filter_map(|c| {
            let v = c.take();
            c.set(v.clone());
            v
        })
    }
}

impl<T: PartialEq> CellSet<T> {
    pub fn remove_first(&self, elem: T) -> Option<usize> {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.get_unchecked(i) };
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

impl<T: Clone> Clone for CellSet<T> {
    #[must_use]
    fn clone(&self) -> Self {
        let mut slots = Vec::with_capacity(self.slots.capacity());
        slots.resize_with(self.first.get(), Default::default);

        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.get_unchecked(i) };
            let e = unsafe { c.take().unwrap_unchecked() };
            c.set(Some(e.clone()));
            slots.push(Cell::new(Some(e)));
        }

        Self {
            slots,
            first: self.first.clone(),
        }
    }
}

impl<T> Clear for CellSet<T> {
    fn clear(&self) {
        for i in self.first.get()..self.capacity() {
            let c = unsafe { self.slots.get_unchecked(i) };
            c.set(None);
        }

        self.first.set(self.capacity());
    }
}

impl<T> Debug for CellSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("CellSet").finish() }
}

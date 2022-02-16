use crate::clear::Clear;
use std::{cell::Cell, mem::MaybeUninit};

type Slot<T> = Cell<Option<T>>;

pub struct FixedCellSet<T, const CAP: usize> {
    slots: [Slot<T>; CAP],
    first: Cell<usize>,
}

impl<T, const CAP: usize> Default for FixedCellSet<T, CAP> {
    #[must_use]
    fn default() -> Self {
        let mut slots: MaybeUninit<[Cell<Option<T>>; CAP]> = MaybeUninit::uninit();

        unsafe {
            for i in 0..CAP {
                (*slots.as_mut_ptr())[i] = Cell::default();
            }

            Self {
                slots: slots.assume_init(),
                first: Cell::new(CAP),
            }
        }
    }
}

impl<T, const CAP: usize> FixedCellSet<T, CAP> {
    #[must_use]
    pub fn capacity(&self) -> usize { CAP }

    #[must_use]
    pub fn len(&self) -> usize { CAP - self.first.get() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.first.get() == CAP }

    pub fn remove(&self, index: usize) -> T {
        let first = self.first.get();
        let slot = &self.slots[first + index];
        let elem = slot.take().unwrap();
        slot.set(self.slots[first].take());
        self.first.set(first + 1);
        elem
    }

    #[must_use]
    pub fn insert(&self, elem: T) -> Option<usize> {
        if self.len() >= CAP {
            return None;
        }

        let index = self.first.get() - 1;
        self.first.set(index);
        let slot = &self.slots[index];
        slot.set(Some(elem));
        Some(0)
    }
}

impl<T: Copy, const CAP: usize> FixedCellSet<T, CAP> {
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> { self.slots.get(self.first.get() + index).and_then(|s| s.get()) }
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ { self.slots.iter().filter_map(|c| c.get()) }

    pub fn retain<F>(&self, f: impl Fn(&T) -> bool) {
        for i in self.first.get()..CAP {
            if !f(&self.slots[i].get().unwrap()) {
                self.remove(i);
            }
        }
    }
}

impl<T: PartialEq + Copy, const CAP: usize> FixedCellSet<T, CAP> {
    pub fn remove_first(&self, elem: T) -> Option<usize> {
        for (i, e) in self.iter().enumerate() {
            if e == elem {
                self.remove(i);
                return Some(i);
            }
        }

        None
    }
}

impl<T: Copy, const CAP: usize> Clone for FixedCellSet<T, CAP> {
    #[must_use]
    fn clone(&self) -> Self {
        Self {
            slots: self.slots.clone(),
            first: self.first.clone(),
        }
    }
}

impl<T, const CAP: usize> Clear for FixedCellSet<T, CAP> {
    fn clear(&self) {
        for s in self.slots.iter() {
            s.set(None)
        }

        self.first.set(CAP);
    }
}

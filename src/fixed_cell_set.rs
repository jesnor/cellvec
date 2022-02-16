use crate::clear::Clear;
use std::cell::Cell;

type Slot<T> = Cell<Option<T>>;

pub struct FixedCellSet<T, const CAP: usize> {
    slots: [Slot<T>; CAP],
    first: Cell<usize>,
}

impl<T: Sized, const CAP: usize> Default for FixedCellSet<T, CAP> {
    #[must_use]
    fn default() -> Self {
        Self {
            slots: array_init::array_init(|_| Cell::default()),
            first: Cell::new(CAP),
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
        let elem = unsafe { slot.take().unwrap_unchecked() };
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

    pub fn retain<F>(&self, f: impl Fn(&T) -> bool) {
        for i in self.first.get()..CAP {
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

impl<T: Copy, const CAP: usize> FixedCellSet<T, CAP> {
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> { self.slots.get(self.first.get() + index).and_then(|s| s.get()) }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ { self.slots.iter().filter_map(|c| c.get()) }
}

impl<T: Clone, const CAP: usize> FixedCellSet<T, CAP> {
    #[must_use]
    pub fn get_clone(&self, index: usize) -> Option<T> {
        self.slots.get(self.first.get() + index).and_then(|s| {
            let v = s.take();
            s.set(v.clone());
            v
        })
    }

    pub fn iter_clone(&self) -> impl Iterator<Item = T> + '_ {
        self.slots.iter().filter_map(|c| {
            let v = c.take();
            c.set(v.clone());
            v
        })
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

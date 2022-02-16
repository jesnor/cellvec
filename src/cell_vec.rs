use crate::clear::Clear;
use std::fmt::Debug;
use std::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
    ops::Deref,
};

struct Slot<T> {
    elem:    UnsafeCell<MaybeUninit<T>>,
    version: Cell<i32>,
}

impl<T> Slot<T> {
    #[must_use]
    fn new(version: i32) -> Self {
        Self {
            elem:    UnsafeCell::new(MaybeUninit::uninit()),
            version: Cell::new(version),
        }
    }

    #[must_use]
    unsafe fn get(&self) -> &T { (*self.elem.get()).assume_init_ref() }
}

pub struct CellVecRef<'t, T> {
    slot:    &'t Slot<T>,
    version: i32,
}

impl<'t, T> CellVecRef<'t, T> {
    #[must_use]
    fn new(slot: &'t Slot<T>, version: i32) -> Self { Self { slot, version } }

    #[must_use]
    pub fn is_valid(&self) -> bool { self.version == self.slot.version.get() }

    #[must_use]
    pub fn get(&self) -> Option<&'t T> {
        if self.version == self.slot.version.get() {
            Some(unsafe { self.slot.get() })
        } else {
            None
        }
    }
}

impl<'t, T> Deref for CellVecRef<'t, T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target { unsafe { self.slot.get() } }
}

impl<'t, T> PartialEq for CellVecRef<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.slot as *const Slot<T>, other.slot as *const Slot<T>) && self.version == other.version
    }
}

impl<'t, T> Eq for CellVecRef<'t, T> {}

impl<'t, T> Clone for CellVecRef<'t, T> {
    #[must_use]
    fn clone(&self) -> Self {
        Self {
            slot:    self.slot,
            version: self.version,
        }
    }
}

impl<'t, T> Copy for CellVecRef<'t, T> {}

impl<'t, T> std::hash::Hash for CellVecRef<'t, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.slot as *const Slot<T>).hash(state);
        self.version.hash(state);
    }
}

impl<'t, T> Debug for CellVecRef<'t, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellVecRef")
            .field("slot", &(self.slot as *const Slot<T>))
            .field("version", &self.version)
            .finish()
    }
}

pub struct CellVec<T> {
    slots:      Vec<Slot<T>>,
    first_free: Cell<i32>,
    len:        Cell<i32>,
    version:    Cell<i32>,
    last:       Cell<i32>,
}

impl<T> CellVec<T> {
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            slots:      (0..cap).map(|i| Slot::new(-(i as i32 + 2))).collect(),
            first_free: Cell::new(0),
            len:        Cell::new(0),
            version:    Cell::new(-1),
            last:       Cell::new(0),
        }
    }

    pub fn init(&self, f: impl Fn(usize) -> T) {
        for (i, slot) in self.slots.iter().enumerate() {
            unsafe { (*slot.elem.get()).write(f(i)) };
        }

        self.version.set(0);
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.slots.get(index).and_then(|e| if e.version.get() >= 0 { Some(unsafe { e.get() }) } else { None })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().take(self.last.get() as usize).filter_map(|e| {
            if e.version.get() >= 0 {
                Some(unsafe { e.get() })
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn len(&self) -> usize { self.len.get() as usize }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    #[must_use]
    pub fn capacity(&self) -> usize { self.slots.capacity() }

    #[must_use]
    fn index_of(&self, slot: &Slot<T>) -> usize {
        let index = unsafe { (slot as *const Slot<T>).offset_from(self.slots.as_ptr() as *const Slot<T>) };

        if index < 0 || index as usize >= self.slots.len() {
            panic!();
        }

        index as usize
    }

    #[must_use]
    pub fn alloc(&self) -> Option<CellVecRef<T>> {
        let index = self.first_free.get();
        let v = self.version.get();
        assert!(v >= 0); // Make sure we're initialized

        if let Some(slot) = self.slots.get(index as usize) {
            self.first_free.set(-slot.version.get() - 1);
            self.version.set(v.wrapping_add(1) & i32::MAX);
            slot.version.set(v);
            self.last.set(self.last.get().max(index + 1));
            Some(CellVecRef::new(slot, v))
        } else {
            None
        }
    }
}

impl<T: Clear> CellVec<T> {
    pub fn remove(&self, r: &CellVecRef<T>) {
        if r.is_valid() {
            let index = self.index_of(r.slot);
            let ff = self.first_free.get();
            r.slot.version.set(-ff - 1);
            r.clear();
            self.first_free.set(index as i32);
            self.last.set(self.last.get().min(index as i32 + 1));
        }
    }
}

impl<T: Clear> Clear for CellVec<T> {
    fn clear(&self) {
        for i in 0..self.last.get() {
            let slot = unsafe { self.slots.get_unchecked(i as usize) };

            if slot.version.get() >= 0 {
                unsafe { slot.get().clear() };
                slot.version.set(-(i as i32 + 2));
            }
        }

        self.first_free.set(0);
        self.len.set(0);
        self.last.set(0);
    }
}

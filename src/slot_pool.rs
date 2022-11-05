use crate::clear::Clear;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
    ops::Deref,
};

pub struct Slot<T> {
    elem:    UnsafeCell<MaybeUninit<T>>,
    version: Cell<i32>,
}

impl<T> Slot<T> {
    #[must_use]
    unsafe fn get(&self) -> &T { (*self.elem.get()).assume_init_ref() }

    fn default() -> Self {
        Self {
            elem:    UnsafeCell::new(MaybeUninit::uninit()),
            version: Default::default(),
        }
    }
}

pub struct SlotPoolRef<'t, T> {
    slot:    &'t Slot<T>,
    version: i32,
}

impl<'t, T> SlotPoolRef<'t, T> {
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

impl<'t, T> Deref for SlotPoolRef<'t, T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target { unsafe { self.slot.get() } }
}

impl<'t, T> PartialEq for SlotPoolRef<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.slot as *const Slot<T>, other.slot as *const Slot<T>) && self.version == other.version
    }
}

impl<'t, T> Eq for SlotPoolRef<'t, T> {}

impl<'t, T> Clone for SlotPoolRef<'t, T> {
    #[must_use]
    fn clone(&self) -> Self {
        Self {
            slot:    self.slot,
            version: self.version,
        }
    }
}

impl<'t, T> Copy for SlotPoolRef<'t, T> {}

impl<'t, T> std::hash::Hash for SlotPoolRef<'t, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.slot as *const Slot<T>).hash(state);
        self.version.hash(state);
    }
}

impl<'t, T> Debug for SlotPoolRef<'t, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellVecRef")
            .field("slot", &(self.slot as *const Slot<T>))
            .field("version", &self.version)
            .finish()
    }
}

pub struct SlotPool<T, A> {
    slots:      A,
    first_free: Cell<i32>,
    len:        Cell<i32>,
    version:    Cell<i32>,
    last:       Cell<i32>,
    last_init:  Cell<i32>,
    _phantom:   PhantomData<fn(T) -> T>,
}

pub type VecSlotPool<T> = SlotPool<T, Vec<Slot<T>>>;
pub type ArraySlotPool<T, const CAP: usize> = SlotPool<T, [Slot<T>; CAP]>;

impl<T> SlotPool<T, Vec<Slot<T>>> {
    #[must_use]
    pub fn new_vec(cap: usize) -> Self {
        let mut v = Vec::with_capacity(cap);
        v.resize_with(v.capacity(), Slot::default);
        Self::new(v)
    }
}

impl<T, const CAP: usize> SlotPool<T, [Slot<T>; CAP]> {
    #[must_use]
    pub fn new_array() -> Self { Self::new(array_init::array_init(|_| Slot::default())) }
}

impl<T, A: AsRef<[Slot<T>]>> SlotPool<T, A> {
    #[must_use]
    pub fn new(slots: A) -> Self {
        for (i, slot) in slots.as_ref().iter().enumerate() {
            slot.version.set(-(i as i32 + 2));
        }

        Self {
            slots,
            first_free: Cell::new(0),
            len: Cell::new(0),
            version: Cell::new(0),
            last: Cell::new(0),
            last_init: Cell::new(0),
            _phantom: Default::default(),
        }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.slots.as_ref().get(index).and_then(|e| if e.version.get() >= 0 { Some(unsafe { e.get() }) } else { None })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.slots.as_ref().iter().take(self.last.get() as usize).filter_map(|e| {
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
    pub fn capacity(&self) -> usize { self.slots.as_ref().len() }

    #[must_use]
    fn index_of(&self, slot: &Slot<T>) -> usize {
        let index = unsafe { (slot as *const Slot<T>).offset_from(self.slots.as_ref().as_ptr() as *const Slot<T>) };

        if index < 0 || index as usize >= self.capacity() {
            panic!();
        }

        index as usize
    }

    #[must_use]
    pub fn insert(&self, value: T) -> Option<SlotPoolRef<T>> {
        let index = self.first_free.get();

        if let Some(slot) = self.slots.as_ref().get(index as usize) {
            let v = self.version.get();
            self.first_free.set(-slot.version.get() - 1);
            self.version.set(v.wrapping_add(1) & i32::MAX);
            slot.version.set(v);
            self.last.set(self.last.get().max(index + 1));

            if index >= self.last_init.get() {
                self.last_init.set(index);
                unsafe { slot.elem.get().write(MaybeUninit::new(value)) };
            }

            Some(SlotPoolRef::new(slot, v))
        } else {
            None
        }
    }
}

impl<T, A: AsRef<[Slot<T>]>> SlotPool<T, A> {
    pub fn remove(&self, r: &SlotPoolRef<T>) {
        if r.is_valid() {
            let index = self.index_of(r.slot);
            let ff = self.first_free.get();
            r.slot.version.set(-ff - 1);
            unsafe { (*r.slot.elem.get()).assume_init_drop() };
            self.first_free.set(index as i32);
            self.last.set(self.last.get().min(index as i32 + 1));
        }
    }
}

impl<T, A: AsRef<[Slot<T>]>> Clear for SlotPool<T, A> {
    fn clear(&self) {
        for i in 0..self.last.get() {
            let slot = unsafe { self.slots.as_ref().get_unchecked(i as usize) };

            if slot.version.get() >= 0 {
                unsafe { (*slot.elem.get()).assume_init_drop() };
                slot.version.set(-(i + 2));
            }
        }

        self.first_free.set(0);
        self.len.set(0);
        self.last.set(0);
    }
}

impl<T, const CAP: usize> Default for SlotPool<T, [Slot<T>; CAP]> {
    fn default() -> Self { Self::new_array() }
}

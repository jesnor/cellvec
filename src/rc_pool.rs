use crate::cell_trait::CellTrait;
use crate::clear::Clear;
use crate::refs::WeakRefTrait;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
    ops::Deref,
};

pub struct Slot<T> {
    elem:      UnsafeCell<MaybeUninit<T>>,
    version:   Cell<i32>,
    ref_count: Cell<u32>,
}

impl<T> Slot<T> {
    #[must_use]
    unsafe fn get(&self) -> &T { (*self.elem.get()).assume_init_ref() }

    fn default() -> Self {
        Self {
            elem:      UnsafeCell::new(MaybeUninit::uninit()),
            version:   Default::default(),
            ref_count: Default::default(),
        }
    }

    fn drop_elem(&self) {
        if self.ref_count.get() > 0 {
            panic!("Trying to remove item with references!")
        }

        unsafe { (*self.elem.get()).assume_init_drop() };
    }
}

pub struct StrongRef<'t, T> {
    slot: &'t Slot<T>,
}

impl<'t, T> StrongRef<'t, T> {
    fn new(slot: &'t Slot<T>) -> Self {
        slot.ref_count.add(1);
        Self { slot }
    }

    pub fn weak(&self) -> WeakRef<'t, T> { WeakRef::new(self.slot) }
}

impl<'t, T> Clone for StrongRef<'t, T> {
    fn clone(&self) -> Self { Self::new(self.slot) }
}

impl<'t, T> Deref for StrongRef<'t, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { unsafe { self.slot.get() } }
}

impl<'t, T> Drop for StrongRef<'t, T> {
    fn drop(&mut self) { self.slot.ref_count.sub(1) }
}

impl<'t, T> TryFrom<WeakRef<'t, T>> for StrongRef<'t, T> {
    type Error = String;

    fn try_from(value: WeakRef<'t, T>) -> Result<Self, Self::Error> {
        value.get().ok_or_else(|| "Element removed!".into())
    }
}

impl<'t, T> crate::refs::StrongRefTrait for StrongRef<'t, T> {
    type Weak = WeakRef<'t, T>;
    fn downgrade(&self) -> WeakRef<'t, T> { self.weak() }
}

pub struct WeakRef<'t, T> {
    slot:    &'t Slot<T>,
    version: i32,
}

impl<'t, T> WeakRef<'t, T> {
    #[must_use]
    fn new(slot: &'t Slot<T>) -> Self {
        Self {
            slot,
            version: slot.version.get(),
        }
    }

    #[must_use]
    pub fn get(&self) -> Option<StrongRef<'t, T>> {
        if self.is_valid() {
            Some(StrongRef::new(self.slot))
        } else {
            None
        }
    }
}

impl<'t, T> WeakRefTrait for WeakRef<'t, T> {
    type Target = T;
    type Strong = StrongRef<'t, T>;

    #[must_use]
    fn upgrade(&self) -> Option<StrongRef<'t, T>> {
        if self.is_valid() {
            Some(StrongRef::new(self.slot))
        } else {
            None
        }
    }

    #[must_use]
    fn is_valid(&self) -> bool { self.version == self.slot.version.get() }
}

impl<'t, T> From<StrongRef<'t, T>> for WeakRef<'t, T> {
    fn from(r: StrongRef<'t, T>) -> Self { r.weak() }
}

impl<'t, T> PartialEq for WeakRef<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.slot as *const Slot<T>, other.slot as *const Slot<T>) && self.version == other.version
    }
}

impl<'t, T> Eq for WeakRef<'t, T> {}

impl<'t, T> Clone for WeakRef<'t, T> {
    #[must_use]
    fn clone(&self) -> Self {
        Self {
            slot:    self.slot,
            version: self.version,
        }
    }
}

impl<'t, T> Copy for WeakRef<'t, T> {}

impl<'t, T> std::hash::Hash for WeakRef<'t, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.slot as *const Slot<T>).hash(state);
        self.version.hash(state);
    }
}

impl<'t, T> Debug for WeakRef<'t, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellVecRef")
            .field("slot", &(self.slot as *const Slot<T>))
            .field("version", &self.version)
            .finish()
    }
}

pub struct RcPool<T, A> {
    slots:      A,
    first_free: Cell<i32>,
    len:        Cell<i32>,
    version:    Cell<i32>,
    last:       Cell<i32>,
    _phantom:   PhantomData<fn(T) -> T>,
}

pub type VecRcPool<T> = RcPool<T, Vec<Slot<T>>>;
pub type ArrayRcPool<T, const CAP: usize> = RcPool<T, [Slot<T>; CAP]>;

impl<T> RcPool<T, Vec<Slot<T>>> {
    #[must_use]
    pub fn new_vec(cap: usize) -> Self {
        let mut v = Vec::with_capacity(cap);
        v.resize_with(v.capacity(), Slot::default);
        Self::new(v)
    }
}

impl<T, A: AsRef<[Slot<T>]>> RcPool<T, A> {
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
    pub fn insert(&self, value: T) -> Option<StrongRef<T>> {
        let index = self.first_free.get();
        let v = self.version.get();

        if let Some(slot) = self.slots.as_ref().get(index as usize) {
            self.first_free.set(-slot.version.get() - 1);
            self.version.set(v.wrapping_add(1) & i32::MAX);
            slot.version.set(v);
            self.last.set(self.last.get().max(index + 1));
            unsafe { (*slot.elem.get()).write(value) };
            Some(StrongRef::new(slot))
        } else {
            None
        }
    }

    pub fn remove(&self, r: &WeakRef<T>) {
        if r.is_valid() {
            let index = self.index_of(r.slot);
            let ff = self.first_free.get();
            self.first_free.set(index as i32);
            self.last.set(self.last.get().min(index as i32 + 1));
            r.slot.version.set(-ff - 1);
            r.slot.drop_elem();
        }
    }
}

impl<T: Clear, A: AsRef<[Slot<T>]>> Clear for RcPool<T, A> {
    fn clear(&self) {
        let last = self.last.get();
        self.first_free.set(0);
        self.len.set(0);
        self.last.set(0);

        for i in 0..last {
            let slot = unsafe { self.slots.as_ref().get_unchecked(i as usize) };
            slot.version.set(-(i + 2));
            slot.drop_elem();
        }
    }
}

impl<T, A: AsRef<[Slot<T>]> + Default> Default for RcPool<T, A> {
    fn default() -> Self { Self::new(A::default()) }
}

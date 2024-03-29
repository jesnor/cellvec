use std::{
    cell::{Cell, RefCell},
    collections::{hash_map::RandomState, HashMap},
    rc::{Rc, Weak},
};

use crate::{
    mcell::MCell,
    rc_pool::{StrongRef, WeakRef},
};

/// # Safety
/// Only implemented for types where their implementations of the standard library traits doesn't
/// store a reference given as argument to a trait function or access a Cell containing a reference given as argument to a trait function
pub unsafe trait SafeTraits {}

unsafe impl SafeTraits for isize {}
unsafe impl SafeTraits for i8 {}
unsafe impl SafeTraits for i16 {}
unsafe impl SafeTraits for i32 {}
unsafe impl SafeTraits for i64 {}

unsafe impl SafeTraits for usize {}
unsafe impl SafeTraits for u8 {}
unsafe impl SafeTraits for u16 {}
unsafe impl SafeTraits for u32 {}
unsafe impl SafeTraits for u64 {}

unsafe impl SafeTraits for f32 {}
unsafe impl SafeTraits for f64 {}

unsafe impl SafeTraits for String {}
unsafe impl SafeTraits for bool {}

unsafe impl<T> SafeTraits for Rc<T> {}
unsafe impl<T> SafeTraits for Weak<T> {}
unsafe impl<'t, T> SafeTraits for StrongRef<'t, T> {}
unsafe impl<'t, T> SafeTraits for WeakRef<'t, T> {}

unsafe impl<T: SafeTraits> SafeTraits for Option<T> {}
unsafe impl<T: SafeTraits> SafeTraits for Cell<T> {}
unsafe impl<T: SafeTraits> SafeTraits for MCell<T> {}
unsafe impl<T: SafeTraits> SafeTraits for RefCell<T> {}

unsafe impl<K: SafeTraits, V: SafeTraits, S: SafeTraits> SafeTraits for HashMap<K, V, S> {}
unsafe impl<T: SafeTraits> SafeTraits for Vec<T> {}
unsafe impl SafeTraits for RandomState {}

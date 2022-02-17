use std::cell::UnsafeCell;
use std::fmt::{Debug, Display};
use std::mem;

use crate::clear::Clear;

pub struct StringCell {
    value: UnsafeCell<String>,
}

impl StringCell {
    pub fn new(value: String) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn replace(&self, val: String) -> String { mem::replace(unsafe { &mut *self.value.get() }, val) }

    pub fn set(&self, val: String) {
        let old = self.replace(val);
        drop(old);
    }

    pub fn get(&self) -> String { unsafe { &*self.value.get() }.clone() }
    pub fn take(&self) -> String { self.replace(Default::default()) }
}

impl Debug for StringCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringCell").field("value", unsafe { &*self.value.get() }).finish()
    }
}

impl Display for StringCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (unsafe { &*self.value.get() } as &dyn Display).fmt(f)
    }
}

impl Default for StringCell {
    fn default() -> Self { Self::new(Default::default()) }
}

impl Clone for StringCell {
    fn clone(&self) -> Self { Self::new(self.get()) }
}

impl Clear for StringCell {
    fn clear(&self) { self.take(); }
}

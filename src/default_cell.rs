use std::cell::Cell;
use std::fmt::{Debug, Display};

use crate::clear::Clear;

/// Cell for non-copy types with default
pub struct DefaultCell<T> {
    cell: Cell<T>,
}

impl<T> DefaultCell<T> {
    pub fn new(value: T) -> Self { Self { cell: Cell::new(value) } }
    pub fn replace(&self, value: T) -> T { self.cell.replace(value) }
    pub fn set(&self, value: T) { self.cell.set(value) }
}

impl<T: Copy> DefaultCell<T> {
    pub fn get(&self) -> T { self.cell.get() }
}

impl<T: Default> DefaultCell<T> {
    pub fn take(&self) -> T { self.cell.take() }
}

impl<T: Debug + Default> Debug for DefaultCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.take();
        let r = f.debug_struct("RichCell").field("value", &v).finish();
        self.set(v);
        r
    }
}

impl<T: Display + Default> Display for DefaultCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.take();
        let r = v.fmt(f);
        self.set(v);
        r
    }
}

impl<T: Default> Default for DefaultCell<T> {
    fn default() -> Self { Self::new(Default::default()) }
}

impl<T: Clone + Default> Clone for DefaultCell<T> {
    fn clone(&self) -> Self {
        let v = self.take();
        self.set(v.clone());
        Self::new(v)
    }
}

impl<T: Default> Clear for DefaultCell<T> {
    fn clear(&self) { self.take(); }
}

use std::cell::Cell;
use std::fmt::{Debug, Display};

use crate::clear::Clear;

pub struct StringCell {
    cell: Cell<String>,
}

impl StringCell {
    pub fn new(value: &str) -> Self {
        Self {
            cell: Cell::new(value.into()),
        }
    }

    pub fn from_string(value: String) -> Self { Self { cell: Cell::new(value) } }
    pub fn replace(&self, value: &str) -> String { self.cell.replace(value.into()) }
    pub fn set(&self, value: &str) { self.cell.set(value.into()) }

    pub fn get(&self) -> String {
        let v = self.take();
        self.cell.set(v.clone());
        v
    }

    pub fn take(&self) -> String { self.cell.take() }
}

impl Debug for StringCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.take();
        let r = f.debug_struct("StringCell").field("value", &v).finish();
        self.cell.set(v);
        r
    }
}

impl Display for StringCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.take();
        let r = (&v as &dyn Display).fmt(f);
        self.cell.set(v);
        r
    }
}

impl Default for StringCell {
    fn default() -> Self { Self::new(Default::default()) }
}

impl Clone for StringCell {
    fn clone(&self) -> Self {
        let v = self.take();
        self.set(&v);
        Self::new(&v)
    }
}

impl Clear for StringCell {
    fn clear(&self) { self.take(); }
}

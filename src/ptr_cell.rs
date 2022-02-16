use std::fmt::Debug;
use std::hash::Hash;
use std::{cell::Cell, ops::Deref};

pub struct PtrCell<T: ?Sized> {
    cell: Cell<T>,
}

impl<T> PtrCell<T> {
    pub fn new(v: T) -> Self { Self { cell: Cell::new(v) } }
    pub fn set(&self, v: T) { self.cell.set(v) }
}

impl<T: Copy> PtrCell<T> {
    pub fn get(&self) -> T { self.cell.get() }
}

impl<T: ?Sized> Deref for PtrCell<&T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { self.cell.get() }
}

impl<T: ?Sized> Hash for PtrCell<&T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { (self.cell.get() as *const T).hash(state); }
}

impl<T: ?Sized> PartialEq for PtrCell<&T> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.cell.get() as *const T, other.cell.get() as *const T) }
}

impl<T: ?Sized> Eq for PtrCell<&T> {}

impl<T: ?Sized> PartialOrd for PtrCell<&T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.cell.get() as *const T).partial_cmp(&(other.cell.get() as *const T))
    }
}

impl<T: ?Sized> Ord for PtrCell<&T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cell.get() as *const T).cmp(&(other.cell.get() as *const T))
    }
}

impl<T: Copy + ?Sized> Clone for PtrCell<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
        }
    }
}

impl<T> From<T> for PtrCell<T> {
    fn from(value: T) -> Self { Self::new(value) }
}

impl<T: Default + ?Sized> Default for PtrCell<T> {
    fn default() -> Self {
        Self {
            cell: Default::default(),
        }
    }
}

impl<T> Debug for PtrCell<&T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtrCell").field("cell", &(self.cell.get() as *const T)).finish()
    }
}

#[test]
fn test_rcell() {
    struct A {
        x: i32,
    }

    let a = A { x: 10 };
    let c = PtrCell::new(&a);
    assert_eq!(c.x, 10);
}

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use crate::mcell::MCell;

pub trait Clear: ClearMut {
    fn clear(&self);
}

pub trait ClearMut {
    fn clear_mut(&mut self);
}

impl<T: Clear> ClearMut for T {
    fn clear_mut(&mut self) { self.clear() }
}

impl ClearMut for String {
    fn clear_mut(&mut self) { self.clear(); }
}

impl<T> ClearMut for Vec<T> {
    fn clear_mut(&mut self) { self.clear(); }
}

impl<K, V> ClearMut for HashMap<K, V> {
    fn clear_mut(&mut self) { self.clear(); }
}

impl<T: ClearMut> Clear for RefCell<T> {
    fn clear(&self) { self.borrow_mut().clear_mut() }
}

impl<T: Default> Clear for Cell<T> {
    fn clear(&self) { self.take(); }
}

impl<T: Default> Clear for MCell<T> {
    fn clear(&self) { self.take(); }
}

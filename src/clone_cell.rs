use crate::safe_traits::SafeTraits;
use std::cell::UnsafeCell;

pub struct CloneCell<T>(UnsafeCell<T>);

impl<T> CloneCell<T> {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_ref(&self) -> &T { &*self.0.get() }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get(&self) -> *mut T { self.0.get() }

    pub fn into_inner(self) -> T { self.0.into_inner() }
}

impl<T> From<T> for CloneCell<T> {
    fn from(v: T) -> Self { Self(UnsafeCell::new(v)) }
}

impl<T: PartialEq + SafeTraits> PartialEq for CloneCell<T> {
    fn eq(&self, other: &Self) -> bool { unsafe { self.get_ref() == other.get_ref() } }
}

impl<T: Eq + SafeTraits> Eq for CloneCell<T> {}

impl<T: PartialOrd<T> + SafeTraits> PartialOrd<CloneCell<T>> for CloneCell<T> {
    fn partial_cmp(&self, other: &CloneCell<T>) -> Option<std::cmp::Ordering> {
        unsafe { self.get_ref().partial_cmp(other.get_ref()) }
    }
}

impl<T: Ord + SafeTraits> Ord for CloneCell<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { self.get_ref().cmp(other.get_ref()) } }
}

impl<T: std::hash::Hash + SafeTraits> std::hash::Hash for CloneCell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            self.get_ref().hash(state);
        }
    }
}

impl<T: Clone + SafeTraits> Clone for CloneCell<T> {
    fn clone(&self) -> Self { Self(UnsafeCell::new(unsafe { self.get_ref().clone() })) }
}

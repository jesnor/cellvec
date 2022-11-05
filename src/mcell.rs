use std::cell::Cell;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, RemAssign, SubAssign};

use crate::cell_trait::CellTrait;
use crate::safe_traits::SafeTraits;

/// Cell wrapper with some convenience methods
pub struct MCell<T> {
    cell: Cell<T>,
}

impl<T> MCell<T> {
    pub fn new(value: T) -> Self { Self { cell: Cell::new(value) } }
    pub fn replace(&self, val: T) -> T { self.cell.replace(val) }

    pub fn set(&self, val: T) {
        let old = self.replace(val);
        drop(old);
    }
}

impl<T: SafeTraits> MCell<T> {
    /// # Safety
    #[allow(clippy::mut_from_ref)]
    unsafe fn as_mut_ref_unchecked(&self) -> &mut T { &mut *self.cell.as_ptr() }

    /// # Safety
    unsafe fn as_ref_unchecked(&self) -> &T { &*self.cell.as_ptr() }

    pub fn add<Rhs>(&self, rhs: Rhs)
    where
        T: AddAssign<Rhs>,
    {
        unsafe { self.as_mut_ref_unchecked() }.add_assign(rhs)
    }

    pub fn div<Rhs>(&self, rhs: Rhs)
    where
        T: DivAssign<Rhs>,
    {
        unsafe { self.as_mut_ref_unchecked() }.div_assign(rhs)
    }

    pub fn sub<Rhs>(&self, rhs: Rhs)
    where
        T: SubAssign<Rhs>,
    {
        unsafe { self.as_mut_ref_unchecked() }.sub_assign(rhs)
    }

    pub fn mul<Rhs>(&self, rhs: Rhs)
    where
        T: MulAssign<Rhs>,
    {
        unsafe { self.as_mut_ref_unchecked() }.mul_assign(rhs)
    }

    pub fn rem<Rhs>(&self, rhs: Rhs)
    where
        T: RemAssign<Rhs>,
    {
        unsafe { self.as_mut_ref_unchecked() }.rem_assign(rhs)
    }

    pub fn neg(&self)
    where
        T: Neg<Output = T> + Clone,
    {
        self.set(self.get().neg())
    }
}

impl<T: Clone + SafeTraits> MCell<T> {
    pub fn get(&self) -> T { unsafe { self.as_ref_unchecked() }.clone() }
}

impl<T: Default> MCell<T> {
    pub fn take(&self) -> T { self.replace(Default::default()) }
}

impl<T: Debug + SafeTraits> Debug for MCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("MCell({:?})", unsafe { self.as_ref_unchecked() }))
    }
}

impl<T: Display + SafeTraits> Display for MCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (unsafe { self.as_ref_unchecked() } as &dyn Display).fmt(f)
    }
}

impl<T: Clone + SafeTraits> Clone for MCell<T> {
    fn clone(&self) -> Self { Self::new(self.get()) }
}

impl<T: Default> Default for MCell<T> {
    fn default() -> Self { Self::new(Default::default()) }
}

impl<T> From<T> for MCell<T> {
    fn from(value: T) -> Self { Self::new(value) }
}

impl<T> CellTrait<T> for MCell<T> {
    fn as_ptr(&self) -> *mut T { self.cell.as_ptr() }

    fn set(&self, value: T) { self.set(value) }

    fn take(&self) -> T
    where
        T: Default,
    {
        self.take()
    }
}

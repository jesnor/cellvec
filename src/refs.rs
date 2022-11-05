use std::{
    ops::Deref,
    rc::{Rc, Weak},
};

pub trait StrongRefTrait: Deref {
    type Weak: WeakRefTrait<Target = Self::Target>;
    fn downgrade(&self) -> Self::Weak;
}

pub trait WeakRefTrait {
    type Target;
    type Strong: StrongRefTrait<Target = Self::Target>;
    fn upgrade(&self) -> Option<Self::Strong>;
    fn is_valid(&self) -> bool;
}

impl<T> StrongRefTrait for Rc<T> {
    type Weak = Weak<T>;
    fn downgrade(&self) -> Self::Weak { Rc::downgrade(self) }
}

impl<T> WeakRefTrait for Weak<T> {
    type Target = T;
    type Strong = Rc<T>;
    fn upgrade(&self) -> Option<Self::Strong> { self.upgrade() }
    fn is_valid(&self) -> bool { self.strong_count() > 0 }
}

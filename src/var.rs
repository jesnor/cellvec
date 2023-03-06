pub trait Var<T> {
    fn get(&self) -> T
    where
        T: Clone;

    fn set(&self, v: T);
}

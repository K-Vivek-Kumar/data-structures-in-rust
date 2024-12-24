pub trait DataStructure<T> {
    fn new() -> Self;
    fn insert(&mut self, val: T);
    fn search(&self, val: T) -> bool;
    fn delete(&mut self, val: T) -> bool;
    fn print(&self);
}

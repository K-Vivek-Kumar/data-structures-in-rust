pub trait DataStructure<T> {
    fn new() -> Self;
    fn insert(&mut self, val: T);
    // fn search(val: T) -> bool;
    // fn delete(val: T);
    fn print(&self);
}

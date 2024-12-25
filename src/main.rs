mod data_structures;
mod traits;

#[allow(unused_imports)]
use data_structures::{BinarySearchTree, LinkedList};
use traits::DataStructure;

fn main() {
    let mut bst = BinarySearchTree::<i32>::new();
    bst.insert(5);
}

mod data_structures;
mod traits;

use data_structures::LinkedList;
use traits::DataStructure;

fn main() {
    let mut linked_list: LinkedList<i32> = LinkedList::new();
    linked_list.insert(4);
    linked_list.insert(5);
    linked_list.insert(-2);
    linked_list.insert(1);
    linked_list.print();
}

use crate::traits::DataStructure;
use std::fmt::Display;

struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T> DataStructure<T> for LinkedList<T>
where
    T: PartialEq + Display,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, val: T) {
        let new_node = Box::new(LinkedListNode {
            value: val,
            next: None,
        });
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }
        let mut temp = &mut self.head;
        while let Some(ref mut node) = *temp {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            } else {
                temp = &mut node.next;
            }
        }
    }

    fn print(&self) {
        let mut temp = &self.head;
        loop {
            if let Some(node) = temp {
                print!("{} ", node.value);
                temp = &node.next;
            } else {
                println!();
                return;
            }
        }
    }

    fn search(&self, val: T) -> bool {
        let mut temp = &self.head;
        loop {
            if let Some(node) = temp {
                if node.value == val {
                    return true;
                }
                temp = &node.next;
            } else {
                return false;
            }
        }
    }

    fn delete(&mut self, val: T) -> bool {
        if self.head.is_none() {
            return false;
        }

        if let Some(ref mut head_node) = self.head {
            if head_node.value == val {
                self.head = head_node.next.take();
                return true;
            }
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == val {
                    node.next = next_node.next.take();
                    return true;
                }
            }
            current = &mut node.next;
        }

        false
    }
}

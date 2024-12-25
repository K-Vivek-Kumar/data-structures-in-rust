use crate::traits::DataStructure;
use std::fmt::Display;

struct BinarySearchTreeNode<T> {
    value: T,
    left: Option<Box<BinarySearchTreeNode<T>>>,
    right: Option<Box<BinarySearchTreeNode<T>>>,
}

pub struct BinarySearchTree<T> {
    root: Option<Box<BinarySearchTreeNode<T>>>,
}

impl<T> DataStructure<T> for BinarySearchTree<T>
where
    T: PartialOrd + PartialEq + Display,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, val: T) {
        let new_node = Box::new(BinarySearchTreeNode {
            value: val,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut current = &mut self.root;
        while let Some(ref mut node) = current {
            if new_node.value < node.value {
                if node.left.is_none() {
                    node.left = Some(new_node);
                    return;
                } else {
                    current = &mut node.left;
                }
            } else {
                if node.right.is_none() {
                    node.right = Some(new_node);
                    return;
                } else {
                    current = &mut node.right;
                }
            }
        }
    }

    fn search(&self, val: T) -> bool {
        let mut current = &self.root;
        while let Some(ref node) = current {
            if node.value == val {
                return true;
            } else if val < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }

    fn delete(&mut self, _val: T) -> bool {
        todo!()
    }

    fn print(&self) {
        Self::inorder(&self);
        println!();
    }
}

#[allow(dead_code)]
impl<T> BinarySearchTree<T>
where
    T: Display,
{
    pub fn inorder(&self) {
        Self::inorder_recursive(&self.root);
    }

    pub fn preorder(&self) {
        Self::preorder_recursive(&self.root);
    }

    pub fn postorder(&self) {
        Self::postorder_recursive(&self.root);
    }

    pub fn traversal(&self, traversal_fn: fn(&BinarySearchTree<T>)) {
        traversal_fn(self);
        println!();
    }

    fn postorder_recursive(node: &Option<Box<BinarySearchTreeNode<T>>>) {
        if let Some(ref n) = node {
            Self::postorder_recursive(&n.left);
            Self::postorder_recursive(&n.right);
            print!("{} ", n.value);
        }
    }

    fn preorder_recursive(node: &Option<Box<BinarySearchTreeNode<T>>>) {
        if let Some(ref n) = node {
            print!("{} ", n.value);
            Self::preorder_recursive(&n.left);
            Self::preorder_recursive(&n.right);
        }
    }

    fn inorder_recursive(node: &Option<Box<BinarySearchTreeNode<T>>>) {
        if let Some(ref n) = node {
            Self::inorder_recursive(&n.left);
            print!("{} ", n.value);
            Self::inorder_recursive(&n.right);
        }
    }
}

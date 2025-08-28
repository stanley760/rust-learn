use std::{cmp::Ordering, ptr::null_mut};

pub struct BinarySearchTree<T> {
    root: Link<T>,
    size: usize,
}

/// defined  a normal binary search tree
pub struct TreeNode<T> {
    pub data: T,
    pub left: Link<T>,
    pub right: Link<T>,
}

type Link<T> = *mut TreeNode<T>;

impl<T> TreeNode<T> {
    pub fn new(data: T) -> TreeNode<T> {
        TreeNode {
            data,
            left: null_mut(),
            right: null_mut(),
        }
    }

    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        unsafe {
            match value.cmp(&self.data) {
                Ordering::Less => {
                    if self.left.is_null() {
                        self.left = Box::into_raw(Box::new(TreeNode::new(value)));
                    } else {
                        (*self.left).insert(value);
                    }
                }
                Ordering::Greater => {
                    if self.right.is_null() {
                        self.right = Box::into_raw(Box::new(TreeNode::new(value)));
                    } else {
                        (*self.right).insert(value);
                    }
                }
                Ordering::Equal => {
                    panic!("Value already exists, don't insert duplicate")
                }
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: null_mut(),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, value: T) {
        unsafe {
            self.root = Self::insert_value(self.root, value);
            self.size += 1;
        }
    }

    unsafe fn insert_value(node: Link<T>, value: T) -> Link<T> {
        unsafe {
            if node.is_null() {
                Box::into_raw(Box::new(TreeNode::new(value)))
            } else {
                match value.cmp(&(*node).data) {
                    Ordering::Less => (*node).left = Self::insert_value((*node).left, value),
                    Ordering::Greater => (*node).right = Self::insert_value((*node).right, value),
                    Ordering::Equal => {}
                }
                node
            }
        }
    }

   
}

impl<T> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_tree_node() {
        let mut root = TreeNode::new(2);
        assert_eq!(root.data, 2);
        assert_eq!(root.left, null_mut());
        assert_eq!(root.right, null_mut());

        unsafe {
            root.insert(1);

            assert_eq!((*root.left).data, 1);
        }
    }
}

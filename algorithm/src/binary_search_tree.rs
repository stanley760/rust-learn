use std::{cmp::Ordering, collections::VecDeque, ptr::null_mut};

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
    // Preorder Traversal
    // Root -> Left -> Right
    pub fn preorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        result.push(&self.data);
        unsafe {
            if self.left != std::ptr::null_mut() {
                result.append(&mut (*self.left).preorder_traversal());
            }
            if self.right != std::ptr::null_mut() {
                result.append(&mut (*self.right).preorder_traversal());
            }
        }
        result
    }

    // Inorder traversal
    // Left -> Root -> Right
    pub fn inorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        unsafe {
            if self.left != std::ptr::null_mut() {
                result.append(&mut (*self.left).inorder_traversal());
            }

            result.push(&self.data);

            if self.right != std::ptr::null_mut() {
                result.append(&mut (*self.right).inorder_traversal());
            }
        }

        result
    }


    // post order traversal
    // left -> right -> root
    pub fn postorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        unsafe {
            if self.left != std::ptr::null_mut() {
                result.append(&mut (*self.left).postorder_traversal());
            }

            if self.right != std::ptr::null_mut() {
                result.append(&mut (*self.right).postorder_traversal());
            }
            result.push(&self.data);
        }
        result
    }

    // bfs traversal which is the same as level order traversal
    // it uses a queue to store the nodes through which we are
    // traversing the tree in each level
    pub fn bfs_for_each<F>(&self) -> Vec<&T> {
        let mut result = Vec::new();

        if self.left.is_null() && self.right.is_null() {
            result.push(&self.data);
            return result;
        }
        let mut queue = VecDeque::new();

        queue.push_back(self as *const TreeNode<T>);

        unsafe {
            while let Some(ptr) = queue.pop_front() {
                let node = &*ptr;
                result.push(&node.data);

                if !node.left.is_null() {
                    queue.push_back(node.left);
                }
                if !node.right.is_null() {
                    queue.push_back(node.right);
                }
            }
        }
        result
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
            // self.root = Self::insert_value(self.root, value);
            // self.size += 1;
            if self.root.is_null() {
                self.root = Box::into_raw(Box::new(TreeNode::new(value)));
                self.size += 1;
                return;
            }

            let mut current = self.root;

            loop {
                match value.cmp(&(*current).data) {
                    Ordering::Less => {
                        if (*current).left.is_null() {
                            (*current).left = Box::into_raw(Box::new(TreeNode::new(value)));
                            self.size += 1;
                            return;
                        }
                        current = (*current).left;
                    }
                    Ordering::Greater => {
                        if (*current).right.is_null() {
                            (*current).right = Box::into_raw(Box::new(TreeNode::new(value)));
                            self.size += 1;
                            break;
                        } else {
                            current = (*current).right;
                        }
                    }
                    Ordering::Equal => {
                        break;
                    }
                }
            }
        }
    }

    // unsafe fn insert_value(node: Link<T>, value: T) -> Link<T> {
    //     if node.is_null() {
    //         Box::into_raw(Box::new(TreeNode::new(value)))
    //     } else {
    //         match value.cmp(&(*node).data) {
    //             Ordering::Less => (*node).left = Self::insert_value((*node).left, value),
    //             Ordering::Greater => (*node).right = Self::insert_value((*node).right, value),
    //             Ordering::Equal => {}
    //         }
    //         node
    //     }
    // }
    // BFS level order traversal
    pub fn bfs_for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        if self.root.is_null() {
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back(self.root);

        unsafe {
            while let Some(ptr) = queue.pop_front() {
                let node = &*ptr;
                f(&node.data);

                if !node.left.is_null() {
                    queue.push_back(node.left);
                }
                if !node.right.is_null() {
                    queue.push_back(node.right);
                }
            }
        }
    }
    // DFS  preorder traversal
    pub fn preorder(&self) -> Vec<&T> {
        if self.root.is_null() {
            return Vec::new();
        }
        unsafe { (*self.root).preorder_traversal() }
    }

    // DFS  inorder traversal
    pub fn inorder(&self) -> Vec<&T> {
        if self.root.is_null() {
            return Vec::new();
        }
        unsafe { (*self.root).inorder_traversal() }
    }

    // DFS  postorder traversal
    pub fn postorder(&self) -> Vec<&T> {
        if self.root.is_null() {
            return Vec::new();
        }
        unsafe { (*self.root).postorder_traversal() }
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

    #[test]
    fn test_bfs_traversal() {
        let mut bst = BinarySearchTree::new();

        // 构建测试树:     4
        //              /   \
        //             2     6
        //            / \   / \
        //           1   3 5   7
        bst.insert(4);
        bst.insert(2);
        bst.insert(6);
        bst.insert(1);
        bst.insert(3);
        bst.insert(5);
        bst.insert(7);

        let mut result = Vec::new();
        bst.bfs_for_each(|value| result.push(*value));
        let expected = vec![4, 2, 6, 1, 3, 5, 7];

        assert_eq!(result, expected);
        assert_eq!(bst.len(), 7);

        // 前序遍历：根 -> 左 -> 右
        let preorder_result = bst.preorder();
        let expected_preorder = vec![&4, &2, &1, &3, &6, &5, &7];
        assert_eq!(preorder_result, expected_preorder);


        // 中序遍历：左 -> 根 -> 右
        let inorder_result = bst.inorder();
        let expected_inorder_result = vec![&1, &2, &3, &4, &5, &6, &7];
        assert_eq!(inorder_result, expected_inorder_result);


        // 后序遍历：左 -> 右 -> 根
        let postorder_result = bst.postorder();
        let expected_postorder_result = vec![&1, &3, &2, &5, &7, &6, &4];
        assert_eq!(postorder_result, expected_postorder_result);
    }
}

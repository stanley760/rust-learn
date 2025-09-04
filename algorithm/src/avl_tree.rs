use std::{cmp::{Ordering, max}, mem, ops::Not};

struct TreeNode<T: Ord> {
    value: T,
    height: usize,
    left: *mut TreeNode<T>,
    right: *mut TreeNode<T>,
}

pub struct AVLTree<T: Ord> {
    root: *mut TreeNode<T>,
    length: usize,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self {
            root: std::ptr::null_mut(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn insert(&mut self, value: T) -> bool{
        unsafe {
            let res = insert_node(&mut self.root, value);
            if res {
                self.length += 1;
            }
            res
        }
    }
    
    pub fn remove(&mut self, value: &T) -> bool {
        // 删除操作的实现
        unsafe {
            let res = remove_node(&mut self.root, value);
            if res {
                self.length -= 1;
            }
            res
        }
    }

}

impl<T: Ord > TreeNode<T> {
    fn child(&self, side: Side) -> *mut TreeNode<T> {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }

    fn child_mut(&mut self, side: Side) -> &mut *mut TreeNode<T> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn height(&self, side: Side) -> usize {
        let child = self.child(side);
        if child.is_null() {
            0
        } else {
            unsafe { (*child).height }
        }
    }

    fn update_height(&mut self) {
        let left_height = self.height(Side::Left);
        let right_height = self.height(Side::Right);
        self.height = 1 + max(left_height, right_height);
    }

    fn balance_factor(&self) -> isize {
        let left_height = self.height(Side::Left) as isize;
        let right_height = self.height(Side::Right) as isize;
        -(left_height - right_height)
    }

    unsafe fn rotate(&mut self, side: Side) {
        let child = self.child_mut(!side);
        if let Some(child) = child.as_mut() {
            // 现在 child 是 &mut TreeNode<T> 类型
            *self.child_mut(!side) = child.child(side);
            mem::swap(self, child);
            *self.child_mut(side) = self as *mut TreeNode<T>;
            self.update_height();
        }
    }

    unsafe fn rebalance(&mut self) {
        self.update_height();
        let balance = self.balance_factor();
        match balance {
            -2 => { // 左边过重
                let left_child = &mut *self.left;
                if left_child.balance_factor() > 0 {
                    // 左-右情况：先左旋左子节点
                    left_child.rotate(Side::Left);
                }
                // 右旋当前节点
                self.rotate(Side::Right);
            },
            2 => { // 右边过重
                let right_child = &mut *self.right;
                if right_child.balance_factor() < 0 {
                    // 右-左情况：先右旋右子节点
                    right_child.rotate(Side::Right);
                }
                // 左旋当前节点
                self.rotate(Side::Left);
            },
            _ => (), // 平衡因子在 -1、0、1 范围内，不需要平衡
        }
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

unsafe fn insert_node<T: Ord>(root:&mut *mut TreeNode<T>, value: T) -> bool {
    if root.is_null() {
        // 创建新节点
        let new_node = Box::new(TreeNode {
            value,
            height: 1,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        });
        *root = Box::into_raw(new_node);
        true
    } else {
        let node = &mut **root;
        let inserted = match value.cmp(&node.value) {
            std::cmp::Ordering::Less => insert_node(&mut node.left, value),
            std::cmp::Ordering::Greater => insert_node(&mut node.right, value),
            std::cmp::Ordering::Equal => false, // 重复值不插入
        };

        if inserted {
            // 更新高度并重新平衡
            node.update_height();
            node.rebalance();
        }
        inserted
    }
}

unsafe fn remove_node<T: Ord>(root: &mut *mut TreeNode<T>, value: &T) -> bool {
    // 删除节点的实现
    if root.is_null() {
        return false; // 节点不存在
    }
    let node = &mut **root;
    let removed = match value.cmp(&node.value)  {
        Ordering::Less => remove_node(&mut node.left, value),
        Ordering::Greater => remove_node(&mut node.right, value),
        Ordering::Equal => {
            // 处理找到要删除的节点的情况
            // todo 
            todo!(); 
        }
    };
    if removed {
        node.update_height();
        node.rebalance();
    }
    removed
}


#[cfg(test)]
mod tests {
    use crate::AVLTree;


    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        assert!(tree.insert(10));
        assert!(tree.insert(20));
    }
}
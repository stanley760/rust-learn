use std::rc::Rc;
use std::cell::RefCell;
#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let mut left_depth = 0;
        let mut right_depth = 0;
        let mut left = root.clone();
        let mut right = root.clone();
        while let Some(node) = left {
            left_depth += 1;
            left = node.borrow().left.clone();
        }
        while let Some(node) = right {
            right_depth += 1;
            right = node.borrow().right.clone();
        }
        if left_depth == right_depth {
           // 完全二叉树
            (1 << left_depth) - 1
        } else {
            // 递归计算左右子树节点数
            1 + Self::count_nodes(root.as_ref().unwrap().borrow().left.clone()) +
                Self::count_nodes(root.as_ref().unwrap().borrow().right.clone())
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[allow(dead_code)]
impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

//  cargo test --package algorithm --lib -- binary::count_complete_tree_nodes::tests --nocapture
#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_count_nodes() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
        })));

        assert_eq!(Solution::count_nodes(root), 6);
    }

    #[test]
    fn test_count_nodes_empty() {
        let root = None;
        assert_eq!(Solution::count_nodes(root), 0);
    }

    #[test]
    fn test_count_nodes_single() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::count_nodes(root), 1);
    }
}
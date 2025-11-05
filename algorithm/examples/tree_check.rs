// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let (val, left, right) = {
            let r = root.borrow();
            (r.val, r.left.clone(), r.right.clone())
        };

        let left = left.unwrap().borrow().val;
        let right = right.unwrap().borrow().val;
        val == left + right
    }
}

fn main() {
    let mut node = TreeNode::new(10);
    let mut node2 = node.clone();
    node.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    node.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let root = Some(Rc::new(RefCell::new(node)));

    assert_eq!(Solution::check_tree(root), true);

    node2.left = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    node2.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let root = Some(Rc::new(RefCell::new(node2)));
    assert_eq!(Solution::check_tree(root), false);
}
use std::{cell::RefCell, rc::Rc};

use crate::ListNode;

pub struct Stack<T> {
    first: Option<Rc<RefCell<ListNode<T>>>>,
    size: usize,
}
#[allow(dead_code)]
impl<T: Copy + PartialEq + Sized> Stack<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn push(&mut self, val: T) {
        let node = ListNode::new(val);
        let new_node = Rc::new(RefCell::new(node));
        new_node.borrow_mut().next = self.first.take();
        self.first = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|ex_first| {
            self.first = ex_first.borrow_mut().next.take();
            self.size -= 1;
            ex_first.borrow().val
        })
    }

    pub fn peek(&self) -> Option<&Rc<RefCell<ListNode<T>>>> {
        self.first.as_ref()
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut current = self.first.clone();
        while let Some(node) = current {
            let (val, next) = {
                let node_ref = node.borrow();
                (node_ref.val, node_ref.next.clone())
            };
            res.push(val);
            current = next;
        }
        res
    }
}

impl<T: Copy + PartialEq + Sized + Default> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

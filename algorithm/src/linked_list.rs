use std::{cell::RefCell, rc::Rc};

/// 链表（linked list）是一种线性数据结构，其中的每个元素都是一个节点对象，
/// 各个节点通过“引用”相连接。引用记录了下一个节点的内存地址，
/// 通过它可以从当前节点访问到下一个节点。
/// 
/// 链表的设计使得各个节点可以分散存储在内存各处，它们的内存地址无须连续。

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T:Clone> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    #[allow(non_snake_case)]
    pub fn insertVal(node: &mut ListNode<T>, val: T)
    where
        T: Into<T>,
    {
        let new_node = Rc::new(RefCell::new(ListNode::new(val.into())));
        new_node.borrow_mut().next = node.next.take();
        node.next = Some(new_node);
    }

    pub fn insert(n0: &Rc<RefCell<ListNode<T>>>, new0: Rc<RefCell<ListNode<T>>>) {
        new0.borrow_mut().next = n0.borrow_mut().next.take();
        n0.borrow_mut().next = Some(new0);
    }

    pub fn remove(prev: &Rc<RefCell<ListNode<T>>>) {
        let p = prev.borrow_mut().next.take();
        if let Some(node) = p {
            prev.borrow_mut().next = node.borrow_mut().next.take();
        }
    }

    pub fn to_vec(n0: &Rc<RefCell<ListNode<T>>>) -> Vec<T> {
        let mut res = Vec::new();
        let mut current = Some(n0.clone());
        while let Some(node) = current {
            res.push(node.borrow().val.clone());
            current = node.borrow().next.clone();
        }
        res
    }
}




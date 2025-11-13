use std::{cell::RefCell, rc::Rc};

/// 链表（linked list）是一种线性数据结构，其中的每个元素都是一个节点对象，
/// 各个节点通过“引用”相连接。引用记录了下一个节点的内存地址，
/// 通过它可以从当前节点访问到下一个节点。
///
/// 链表的设计使得各个节点可以分散存储在内存各处，它们的内存地址无须连续。
/// Warning: This code is not recommended. use [unsafe_linked_queue]
///
/// # example
/// ```
///  use algorithm::Queue;
///  let mut queue = Queue::new();
///  queue.push(1);
///  queue.push(2);
///  queue.push(3);
///  let mut iter = queue.iter();
///  assert_eq!(iter.next(), Some(&1));
///  assert_eq!(iter.next(), Some(&2));
///  assert_eq!(iter.next(), Some(&3));
///  assert_eq!(iter.next(), None);
/// ```
///

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Clone + PartialEq> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    #[allow(non_snake_case)]
    pub fn insertVal(node: &mut ListNode<T>, val: T)
    where
        T: Into<T>,
    {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
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

    pub fn access(n0: &Rc<RefCell<ListNode<T>>>, index: usize) -> Option<T> {
        let mut current = Some(n0.clone());
        for _ in 0..index {
            if let Some(node) = current {
                current = node.borrow().next.clone();
            } else {
                return None;
            }
        }
        current.map(|node| node.borrow().val.clone())
    }

    pub fn find(n0: &Rc<RefCell<ListNode<T>>>, val: T) -> Option<Rc<RefCell<ListNode<T>>>> {
        let mut current = Some(n0.clone());
        while let Some(node) = current {
            if node.borrow().val == val {
                return Some(node);
            }
            current = node.borrow().next.clone();
        }
        None
    }
}

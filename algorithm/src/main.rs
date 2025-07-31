use std::{cell::RefCell, rc::Rc};

use crate::fibonacci::Fibonacci;

mod fibonacci;

fn main() {
    let mut li = Fibonacci::new();
    let option = li.at(10);
    println!("{}", option);
    let n0 = Rc::new(RefCell::new(ListNode { val: 1, next: None }));
    let n1 = Rc::new(RefCell::new(ListNode { val: 3, next: None }));
    let n2 = Rc::new(RefCell::new(ListNode { val: 2, next: None }));
    let n3 = Rc::new(RefCell::new(ListNode { val: 5, next: None }));
    let n4 = Rc::new(RefCell::new(ListNode { val: 4, next: None }));

    // 构建节点之间的引用
    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());

    let n = access(n0.clone(), 2);
    println!("{:?}", n);
}

#[derive(Debug)]
struct ListNode<T> {
    val: T, // 节点值
    next: Option<Rc<RefCell<ListNode<T>>>>, // 指向下一节点的指针
}

fn access<T>(head: Rc<RefCell<ListNode<T>>>, index: i32) -> Option<Rc<RefCell<ListNode<T>>>> {
    fn dfs<T>(
        head: Option<&Rc<RefCell<ListNode<T>>>>,
        index: i32,
    ) -> Option<Rc<RefCell<ListNode<T>>>> {
        if index <= 0 {
            return head.cloned();
        }

        if let Some(node) = head {
            dfs(node.borrow().next.as_ref(), index - 1)
        } else {
            None
        }
    }

    dfs(Some(head).as_ref(), index)
}

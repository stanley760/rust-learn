use std::{cell::RefCell, rc::Rc};

use algorithm::ListNode;

#[test]
fn test_linked_list() {
    let n0 = Rc::new(RefCell::new(ListNode::new(1)));
    let n1 = Rc::new(RefCell::new(ListNode::new(3)));
    let n2 = Rc::new(RefCell::new(ListNode::new(2)));
    let n3 = Rc::new(RefCell::new(ListNode::new(5)));
    let n4 = Rc::new(RefCell::new(ListNode::new(4)));

    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());
    n4.borrow_mut().next = None;
    assert_eq!(ListNode::to_vec(&n0), vec![1, 3, 2, 5, 4]);
    let n5 = Rc::new(RefCell::new(ListNode::new(6)));
    ListNode::insert(&n1, n5);

    assert_eq!(ListNode::to_vec(&n0), vec![1, 3, 6, 2, 5, 4]);
    ListNode::remove(&n0);
    assert_eq!(ListNode::to_vec(&n0), vec![1, 6, 2, 5, 4]);

    let val = ListNode::access(&n0, 1);
    assert_eq!(val, Some(6));

    let node = ListNode::find(&n0, 6);
    assert_eq!(node.map(|node| node.borrow().val.clone()), Some(6));
}

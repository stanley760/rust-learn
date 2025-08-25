use algorithm::{ArrayStack, Stack};

#[test]
pub fn test_array_stack() {
    let mut stack = ArrayStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&2));
    assert_eq!(stack.to_vec(), vec![1, 2]);
    stack.clear();
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.to_vec(), vec![]);
}

#[test]
pub fn test_list_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek().map(|node| node.borrow().val), Some(2));
    assert_eq!(stack.to_vec(), vec![2, 1]);
}

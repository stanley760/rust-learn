#[test]
pub fn test_vec_impl() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack, vec![1, 2, 3]);

    let pop = stack.pop();
    assert_eq!(pop, Some(3));

    let last = stack.last();
    assert_eq!(last, Some(&2));

    let first = stack.first();
    assert_eq!(first, Some(&1));

    let len = stack.len();
    assert_eq!(len, 2);

    let is_empty = stack.is_empty();
    assert_eq!(is_empty, false);
}
use algorithm::ArrayQueue;

#[test]
fn test_array_queue() {
    let mut queue = ArrayQueue::new(2);
    assert_eq!(queue.capacity(), 2);
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());

    queue.push(1);
    queue.push(2);
    assert_eq!(queue.size(), 2);
    assert!(!queue.is_empty());
    assert_eq!(queue.peek(), Some(1));

    queue.push(3); // triggers resize
    assert_eq!(queue.capacity(), 4);
    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek(), Some(1));

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.peek(), Some(2));

    queue.push(4);
    queue.push(5); // triggers resize
    assert_eq!(queue.capacity(), 4);
    assert_eq!(queue.size(), 4);

    assert_eq!(queue.to_vec(), vec![0, 0, 0, 0, 2, 3, 4, 5]);
}
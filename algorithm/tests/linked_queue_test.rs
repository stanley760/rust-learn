#[cfg(test)]
mod test {
    use algorithm::Queue;

    #[test]
    fn test_unsafe_linked_queue() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.to_vec(), vec![2, 3]);
    }

    #[test]
    fn test_iter() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let mut iter_mut = queue.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), None);

        let mut into_iter = queue.into_iter();
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), None);
    }
}

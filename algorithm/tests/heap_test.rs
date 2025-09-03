#[cfg(test)]
mod tests {
    use algorithm::{Heap, MaxHeap, MinHeap};

    #[test]
    pub fn test_empty_heap() {
        let mut heap: Heap<i32> = MaxHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap: Heap<i32> = MinHeap::new();
        heap.add(3);
        heap.add(1);
        heap.add(2);
        assert_eq!(heap.len(), 3);
        assert!(!heap.is_empty());
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(3));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap: Heap<i32> = MaxHeap::new();
        heap.add(8);
        heap.add(1);
        heap.add(5);
        heap.add(3);
        assert_eq!(heap.len(), 4);
        assert!(!heap.is_empty());
        assert_eq!(heap.next(), Some(8));
        assert_eq!(heap.next(), Some(5));
        assert_eq!(heap.next(), Some(3));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}
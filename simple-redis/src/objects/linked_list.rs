use std::marker::PhantomData;

pub struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>, // Own ownership of Node<T> allocated on the heap.
}

struct Node<T> {
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            next: None,
            prev: None,
            data,
        }
    }

    fn into_data(self: Box<Self>) -> T {
        self.data
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }
}

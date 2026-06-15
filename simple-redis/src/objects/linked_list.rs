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
    ele: T,
}

impl<T> Node<T> {
    fn new(ele: T) -> Self {
        Node {
            next: None,
            prev: None,
            ele,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.ele
    }
}

pub struct Iter<'a, T: 'a> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|head| unsafe { &(*head).ele })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|tail| unsafe { &(*tail).ele })
    }

    pub fn push_front(&mut self, data: T) {
        let node = Box::into_raw(Box::new(Node::new(data)));
        unsafe {
            (*node).next = self.head;
            (*node).prev = None;

            match self.head {
                None => self.tail = Some(node),
                Some(head) => (*head).prev = Some(node),
            }

            self.head = Some(node);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node);
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*head).prev = None,
            }

            self.len -= 1;
            node.into_element()
        })
    }

    pub fn push_back(&mut self, data: T) {
        let node = Box::into_raw(Box::new(Node::new(data)));
        unsafe {
            (*node).prev = self.tail;
            (*node).next = None;

            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node);
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail).next = None,
            }

            self.len -= 1;
            node.into_element()
        })
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn rotate_tail_to_head(&mut self) {
        if self.len <= 1 {
            return;
        }

        unsafe {
            let tail = self.tail.unwrap();
            let prev = (*tail).prev.unwrap();

            (*prev).next = None;
            self.tail = Some(prev);

            (*tail).prev = None;
            (*tail).next = self.head;

            if let Some(head) = self.head {
                (*head).prev = Some(tail);
            }

            self.head = Some(tail);
        }
    }

    pub fn rotate_head_to_tail(&mut self) {
        if self.len <= 1 {
            return;
        }

        unsafe {
            let head = self.head.unwrap();
            let next = (*head).next.unwrap();

            (*next).prev = None;
            self.head = Some(next);

            (*head).next = None;
            (*head).prev = self.tail;

            if let Some(tail) = self.tail {
                (*tail).next = Some(head);
            }

            self.tail = Some(head);
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| unsafe {
            self.len -= 1;
            self.head = (*node).next;
            &(*node).ele
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_pop_front_back() {
        let mut list = LinkedList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push_back(1);
        list.push_back(2);
        list.push_front(0);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&0));
        assert_eq!(list.back(), Some(&2));

        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn rotate_moves_nodes() {
        let mut list = LinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.rotate_tail_to_head();
        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.back(), Some(&2));
        assert_eq!(list.len(), 3);

        list.rotate_head_to_tail();
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
        list.iter().for_each(|f| println!("{}", f));
    }
}

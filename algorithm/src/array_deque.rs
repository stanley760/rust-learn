use std::{marker::PhantomData, ptr::NonNull};

pub struct ArrayDeque<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> Default for ArrayDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ArrayDeque<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> ArrayDeque<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    /// Returns the length of the linked list
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.front = boxed_node.back;
                if let Some(new) = boxed_node.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }
                self.len -= 1;
                result
            })
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|node| unsafe {
            let boxed_node = Box::from_raw(node.as_ptr());
            let result = boxed_node.elem;

            self.back = boxed_node.front;

            if let Some(new) = self.back {
                (*new.as_ptr()).back = None;
            } else {
                self.front = None;
            }
            self.len -= 1;
            result
        })
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn peek_front(&mut self) -> Option<&T> {
        self.front.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn peek_back(&mut self) -> Option<&T> {
        self.back.map(|node| unsafe { &(*node.as_ptr()).elem })
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.front.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.back.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        unsafe {
            (0..index)
                .try_fold(self.front?, |cur, _| (*cur.as_ptr()).back)
                .map(|cur| &(*cur.as_ptr()).elem)
        }
    }

    pub fn find(&self, value: &T) -> Option<usize>
    where
        T: PartialEq<T>,
    {
        unsafe {
            let mut cur = self.front;

            let mut index = 0;

            while let Some(node) = cur {
                if &(*node.as_ptr()).elem == value {
                    return Some(index);
                }
                cur = (*node.as_ptr()).back;
                index += 1;
            }
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        if index == 0 {
            self.pop_front()
        } else if index == self.len - 1 {
            self.pop_back()
        } else {
            unsafe {
                let current = (0..index)
                    .try_fold(self.front.unwrap(), |cur, _| (*cur.as_ptr()).back)
                    .unwrap();

                let prev = (*current.as_ptr()).front.unwrap();
                let next = (*current.as_ptr()).back.unwrap();

                (*prev.as_ptr()).back = Some(next);
                (*next.as_ptr()).front = Some(prev);

                let boxed_node = Box::from_raw(current.as_ptr());
                self.len -= 1;
                Some(boxed_node.elem)
            }
        }
    }

    pub fn remove_value(&mut self, value: &T) -> bool
    where
        T: PartialEq,
    {
        unsafe {
            let mut current = self.front;
            let mut index = 0;

            while let Some(node) = current {
                if (*node.as_ptr()).elem == *value {
                    self.remove(index);
                    return true;
                }
                current = (*node.as_ptr()).back;
                index += 1;
            }
        }
        false
    }
}

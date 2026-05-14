use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};


struct Node {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<Node>>>, // Avoid memory leaks caused by circular references
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key:i32, val:i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { key, val, prev: None, next: None }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    kv_node: HashMap<i32, Rc<RefCell<Node>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));
        dummy.borrow_mut().next = Some(dummy.clone());
        Self { capacity: capacity as usize, dummy, kv_node: HashMap::new() }
    }
    
    fn get_node(&mut self, key:i32) -> Option<Rc<RefCell<Node>>> {
        self.kv_node.get(&key).map(|node| {
            let node = Rc::clone(node);
            Self::remove(&node);
            self.push_front(&node);
            node
        })
    }

    fn get(&mut self, key: i32) -> i32 {
        return match self.get_node(key) {
            Some(node) => node.borrow().val,
            None => -1,
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        
    }


    fn remove(e: &Rc<RefCell<Node>>) {
        // remove the relationship of which current node between current.prev and current.next
        let prev = e.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        let next = e.borrow_mut().next.take().unwrap();
        // connect the prev node to the next node.
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
        prev.borrow_mut().next = Some(next);
    }

    fn push_front(&mut self, e: &Rc<RefCell<Node>>) {
        let next = self.dummy.borrow_mut().next.take().unwrap();
        next.borrow_mut().prev = Some(Rc::downgrade(&e));
        e.borrow_mut().next = Some(next);
        e.borrow_mut().prev = Some(Rc::downgrade(&self.dummy));

        self.dummy.borrow_mut().next = Some(Rc::clone(e));
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
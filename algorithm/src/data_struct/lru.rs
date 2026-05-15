use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub struct Node<K, V> {
    key: K,
    val: V,
    prev: Option<Weak<RefCell<Node<K, V>>>>, // Avoid memory leaks caused by circular references
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}

pub struct LRUCache<K, V> {
    capacity: usize,
    dummy: Rc<RefCell<Node<K, V>>>,
    kv_node: HashMap<K, Rc<RefCell<Node<K, V>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl<K, V> LRUCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + Default,
    V: Clone + Default,
{
    pub fn new(capacity: usize) -> Self {
        let dummy = Node::new(K::default(), V::default());
        dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));
        dummy.borrow_mut().next = Some(dummy.clone());
        Self {
            capacity,
            dummy,
            kv_node: HashMap::new(),
        }
    }

    fn get_node(&mut self, key: &K) -> Option<Rc<RefCell<Node<K, V>>>> {
        return match self.kv_node.get(key) {
            Some(node) => {
                let node = Rc::clone(node);
                Self::remove(&node);
                self.push_front(&node);
                Some(node)
            }
            _ => None,
        };
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        return match self.get_node(key) {
            Some(node) => Some(node.borrow().val.clone()),
            None => None,
        };
    }

    pub fn put(&mut self, key: K, val: V) {
        if let Some(node) = self.get_node(&key) {
            node.borrow_mut().val = val;
            return;
        }
        let node = Node::new(key.clone(), val.clone());
        self.push_front(&node);
        self.kv_node.insert(key, node);
        // out of bound
        if self.kv_node.len() > self.capacity {
            let back_node = self
                .dummy
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap();
            self.kv_node.remove(&back_node.borrow().key.clone());
            Self::remove(&back_node);
        }
    }

    fn remove(e: &Rc<RefCell<Node<K, V>>>) {
        // remove the relationship of which current node between current.prev and current.next
        let prev = e.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        let next = e.borrow_mut().next.take().unwrap();
        // connect the prev node to the next node.
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
        prev.borrow_mut().next = Some(next);
    }

    fn push_front(&mut self, e: &Rc<RefCell<Node<K, V>>>) {
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
#[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru: LRUCache<_, _> = LRUCache::new(2);
        lru.put(1, 1); // 缓存是 {1=1}
        lru.put(2, 2); // 缓存是 {1=1, 2=2}
        assert_eq!(Some(1), lru.get(&1));    // 返回 1
        lru.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
        assert_eq!(None, lru.get(&2));    // 返回 -1 (未找到)
        lru.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
        assert_eq!(None, lru.get(&1));    // 返回 -1 (未找到)
        assert_eq!(Some(3), lru.get(&3));    // 返回 3
        assert_eq!(Some(4), lru.get(&4));    // 返回 4

    }
}

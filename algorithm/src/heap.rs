
#[derive(Debug)]
pub struct Heap<T: Default> {
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Default> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn add(&mut self, val: T) {
        self.count += 1;
        self.items.push(val);

        let mut idx = self.count;
        // exists parent and current node is less than parent node.
        while self.parent_idx(idx) > 0 {
            // get parent index.
            let parent_idx = self.parent_idx(idx);
            // if current node is less than parent node, swap them.
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
            }
            // update index to parent index. 
            idx = parent_idx;    
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }
    
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let left_child_idx = self.left_child_idx(idx);
            let right_child_idx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
                left_child_idx
            } else {
                right_child_idx
            }
        }
    }
}

impl<T: Default> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }
        let next = Some(self.items.swap_remove(1));
        self.count -= 1;

        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let child_idx = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                    self.items.swap(idx, child_idx);
                }
                idx = child_idx;
            }
        }
        next
    }

    
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T: Default + Ord>() -> Heap<T> {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T: Default + Ord>() -> Heap<T> {
        Heap::new(|a, b| a > b)
    }
}
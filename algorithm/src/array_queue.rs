// the queue is implemented by the circlular array
// the front of the queue is the element at the front of the array
// the rear of the queue is the element at the end of the array

pub struct ArrayQueue<T> {
    data: Vec<T>,
    front: i32,
    length: i32,
    capacity: i32,
}

impl<T: Copy + Default> ArrayQueue<T> {
    pub fn new(capacity: i32) -> Self {
        Self {
            data: vec![T::default(); capacity as usize],
            front: 0,
            length: 0,
            capacity,
        }
    }

    pub fn capacity(&self) -> i32 {
        self.capacity
    }

    pub fn size(&self) -> i32 {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn push(&mut self, val: T) {
        if self.length == self.capacity {
            self.resize((self.capacity * 2) as usize);
        }
        // calculate the index of the rear element
        let rear = (self.front + self.length) % self.capacity;
        self.data[rear as usize] = val;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let num = self.peek();
        self.front = (self.front + 1) % self.capacity;
        self.length -= 1;
        num
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.data[self.front as usize])
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_data = vec![T::default(); new_capacity];
        for i in 0..self.length {
            new_data[i as usize] = self.data[((self.front + i) % self.capacity) as usize];
        }
        self.data = new_data;
        self.front = 0;
        self.capacity = new_capacity as i32;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let cap = self.capacity;
        let mut front = self.front;
        let mut res = vec![T::default(); cap as usize];
        for _ in 0..self.length {
            res.push(self.data[(front % self.capacity) as usize]);
            front += 1;
        }
        res
    }
}

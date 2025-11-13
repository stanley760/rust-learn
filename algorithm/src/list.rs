#[derive(Debug)]
pub struct MyList {
    arr: Vec<i32>,
    cap: usize,
    size: usize,
    extend_ratio: usize, // 每次列表扩容的倍数
}

#[allow(unused, unused_comparisons)]
impl MyList {
    pub fn new(cap: usize) -> Self {
        let mut vec = vec![0; cap];
        Self {
            arr: vec,
            cap,
            size: 0,
            extend_ratio: 2,
        }
    }

    /* 获取列表长度（当前元素数量）*/
    pub fn size(&self) -> usize {
        return self.size;
    }

    /* 获取列表容量 */
    pub fn capacity(&self) -> usize {
        return self.cap;
    }

    /* 访问元素 */
    pub fn get(&self, index: usize) -> i32 {
        if index >= self.size {
            panic!("索引越界")
        }
        return self.arr[index];
    }

    pub fn set(&mut self, index: usize, val: i32) {
        if index >= self.size {
            panic!("索引越界")
        }
        self.arr[index] = val;
    }

    pub fn add(&mut self, val: i32) {
        if self.size == self.cap {
            self.extend_capacity();
        }
        self.arr[self.size] = val;
        self.size += 1;
    }

    pub fn insert(&mut self, index: usize, val: i32) {
        if index > self.size {
            panic!("索引越界")
        }
        if self.size == self.cap {
            self.extend_capacity();
        }
        for i in (index..self.size).rev() {
            self.arr[i + 1] = self.arr[i];
        }
        self.arr[index] = val;
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> i32 {
        if index >= self.size {
            panic!("索引越界")
        }

        let num = self.arr[index];
        for i in index..self.size - 1 {
            self.arr[i] = self.arr[i + 1];
        }
        self.size -= 1;
        num
    }

    pub fn extend_capacity(&mut self) {
        let new_capacity = self.cap * self.extend_ratio;
        let mut new_arr = Vec::with_capacity(new_capacity);
        for i in 0..self.size {
            new_arr.push(self.arr[i]);
        }
        self.arr = new_arr;
        self.arr.resize(new_capacity, 0);
        self.cap = new_capacity;
    }

    pub fn to_array(&self) -> Vec<i32> {
        let mut vec = Vec::with_capacity(self.size);
        for i in 0..self.size {
            vec.push(self.arr[i]);
        }
        vec
    }
}

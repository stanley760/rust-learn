// hash table chaining

use crate::Pair;

pub struct HashTableChaining {
    size: usize,
    capacity: usize,
    load_thres: f32,
    extend_ratio: usize,
    buckets: Vec<Vec<Pair>>,
}

impl HashTableChaining {
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: 4,
            load_thres: 0.75,
            extend_ratio: 2,
            buckets: vec![vec![]; 4],
        }
    }

    fn hash_func(&self, key: i32) -> usize {
        key as usize % self.capacity
    }

    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    pub fn remove(&mut self, key: i32) -> Option<String> {
        let idx = self.hash_func(key);
        for (i, pair) in self.buckets[idx].iter_mut().enumerate() {
            if pair.key == key {
                let pair = self.buckets[idx].remove(i);
                self.size -= 1;
                return Some(pair.value);
            }
        }
        None
    }
    // extend hash table.
    pub fn extend(&mut self) {
        // original hash table.
        let buckets_tmp = std::mem::take(&mut self.buckets);

        // init new hash table.
        self.capacity *= self.extend_ratio;
        self.buckets = vec![Vec::new(); self.capacity as usize];
        self.size = 0;

        // rehash.
        for bucket in buckets_tmp {
            for pair in bucket {
                self.put(pair.key, pair.value);
            }
        }
    }

    pub fn put(&mut self, key: i32, val: String) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }
        let idx = self.hash_func(key);
        // loop buckets. if key exists, update value.
        for pair in self.buckets[idx].iter_mut() {
            if pair.key == key {
                pair.value = val;
                return;
            }
        }
        // if key not exists, insert.
        self.buckets[idx].push(Pair { key, value: val });
        self.size += 1;
    }
    // get value by key.
    pub fn get(&self, key: i32) -> Option<&str> {
        let idx = self.hash_func(key);
        // loop buckets. if key exists, return value.
        for pair in self.buckets[idx].iter() {
            if pair.key == key {
                return Some(&pair.value);
            }
        }
        // if key not exists, return None.
        None
    }

    pub fn print(&self) {
        for bucket in self.buckets.iter() {
            for pair in bucket.iter() {
                println!("{}: {}", pair.key, pair.value);
            }
        }
    }
}

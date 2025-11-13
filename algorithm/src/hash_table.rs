#[derive(Debug, PartialEq, Clone)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

pub struct ArrayHashTable {
    buckets: Vec<Option<Pair>>,
}

impl ArrayHashTable {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; 10],
        }
    }

    pub fn hash_func(&self, key: i32) -> usize {
        (key % self.buckets.len() as i32) as usize
    }

    pub fn get(&self, key: i32) -> Option<&String> {
        let index = self.hash_func(key);
        self.buckets[index].as_ref().map(|p| &p.value)
    }

    pub fn put(&mut self, key: i32, value: &str) {
        let index = self.hash_func(key);
        self.buckets[index] = Some(Pair {
            key,
            value: value.to_string(),
        });
    }

    pub fn remove(&mut self, key: i32) {
        let index = self.hash_func(key);
        self.buckets[index] = None;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pair> {
        self.buckets.iter().filter_map(|p| p.as_ref())
    }

    pub fn entry_set(&self) -> Vec<&Pair> {
        self.iter().collect()
    }

    pub fn value_set(&self) -> Vec<&String> {
        self.iter().map(|p| &p.value).collect()
    }

    pub fn print(&self) {
        for p in self.iter() {
            println!("key:{:?}, value:{:?}", p.key, p.value);
        }
    }
}

impl Default for ArrayHashTable {
    fn default() -> Self {
        Self::new()
    }
}

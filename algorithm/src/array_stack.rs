pub struct ArrayStack<T> {
    data: Vec<T>,
}

impl<T: Copy + PartialEq + Sized> ArrayStack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            panic!("栈为空")
        };
        self.data.last()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data.to_vec()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

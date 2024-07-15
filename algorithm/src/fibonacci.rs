pub struct Fibonacci {

    cache: Vec<usize>
}

impl Fibonacci {
    pub(crate) fn new() -> Self {
        Fibonacci {
            cache: vec![0, 1, 1]
        }
    }

    pub(crate) fn at(&mut self, n: usize) -> usize {
        return match self.cache.get(n) {
            Some(num) => *num,
            None => {
                let v = self.at(n -1) + self.at(n - 2);
                self.cache.push(v);
                v
            }
        };
    }
}
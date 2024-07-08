struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn invoke() {
    let mut counter = Counter::new();
    println!("next value: {}", counter.next().unwrap());
}

#[test]
fn invoke_test() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    // [1, 2, 3, 4, 5] 和 [2, 3, 4, 5] 的迭代器合并后
    // 新的迭代器形如 [(1, 2),(2, 3),(3, 4),(4, 5)]
    // map -> [2, 6, 12, 20]
    // filter -> [6 ,12]
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}
